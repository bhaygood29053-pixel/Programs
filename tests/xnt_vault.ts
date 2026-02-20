import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("xnt_vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.XntVault as Program;

  const admin = provider.wallet;
  const strategist = Keypair.generate();
  const userA = Keypair.generate();
  const userB = Keypair.generate();

  let mint: PublicKey;
  let vaultPda: PublicKey;
  let vaultAuthorityPda: PublicKey;
  let vaultTokenAccount: PublicKey;
  let userAToken: PublicKey;
  let userBToken: PublicKey;
  let strategistToken: PublicKey;
  let userAPosition: PublicKey;
  let userBPosition: PublicKey;

  const airdrop = async (pubkey: PublicKey) => {
    const sig = await provider.connection.requestAirdrop(pubkey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(sig, "confirmed");
  };

  before(async () => {
    await airdrop(strategist.publicKey);
    await airdrop(userA.publicKey);
    await airdrop(userB.publicKey);

    mint = await createMint(
      provider.connection,
      (admin as any).payer,
      admin.publicKey,
      null,
      6,
    );

    [vaultPda] = PublicKey.findProgramAddressSync([
      Buffer.from("vault"),
      mint.toBuffer(),
    ], program.programId);

    [vaultAuthorityPda] = PublicKey.findProgramAddressSync([
      Buffer.from("vault_authority"),
      vaultPda.toBuffer(),
    ], program.programId);

    vaultTokenAccount = await anchor.utils.token.associatedAddress({
      mint,
      owner: vaultAuthorityPda,
    });

    userAToken = await createAssociatedTokenAccount(provider.connection, (admin as any).payer, mint, userA.publicKey);
    userBToken = await createAssociatedTokenAccount(provider.connection, (admin as any).payer, mint, userB.publicKey);
    strategistToken = await createAssociatedTokenAccount(provider.connection, (admin as any).payer, mint, strategist.publicKey);

    await mintTo(provider.connection, (admin as any).payer, mint, userAToken, admin.publicKey, 1_000_000_000);
    await mintTo(provider.connection, (admin as any).payer, mint, userBToken, admin.publicKey, 1_000_000_000);
    await mintTo(provider.connection, (admin as any).payer, mint, strategistToken, admin.publicKey, 1_000_000_000);

    [userAPosition] = PublicKey.findProgramAddressSync(
      [Buffer.from("position"), vaultPda.toBuffer(), userA.publicKey.toBuffer()],
      program.programId,
    );
    [userBPosition] = PublicKey.findProgramAddressSync(
      [Buffer.from("position"), vaultPda.toBuffer(), userB.publicKey.toBuffer()],
      program.programId,
    );

    await program.methods
      .initializeVault(strategist.publicKey)
      .accounts({
        admin: admin.publicKey,
        vault: vaultPda,
        vaultAuthority: vaultAuthorityPda,
        xntMint: mint,
        vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  });

  it("proves share math and compound flow", async () => {
    await program.methods
      .deposit(new anchor.BN(100_000_000))
      .accounts({
        owner: userA.publicKey,
        vault: vaultPda,
        vaultAuthority: vaultAuthorityPda,
        position: userAPosition,
        ownerTokenAccount: userAToken,
        vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([userA])
      .rpc();

    await program.methods
      .compound(new anchor.BN(100_000_000))
      .accounts({
        strategist: strategist.publicKey,
        vault: vaultPda,
        strategistRewardTokenAccount: strategistToken,
        vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([strategist])
      .rpc();

    await program.methods
      .deposit(new anchor.BN(100_000_000))
      .accounts({
        owner: userB.publicKey,
        vault: vaultPda,
        vaultAuthority: vaultAuthorityPda,
        position: userBPosition,
        ownerTokenAccount: userBToken,
        vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([userB])
      .rpc();

    const vault = await program.account.vault.fetch(vaultPda);
    const positionA = await program.account.position.fetch(userAPosition);
    const positionB = await program.account.position.fetch(userBPosition);

    expect(vault.totalAssets.toNumber()).to.eq(300_000_000);
    expect(vault.totalShares.toNumber()).to.eq(150_000_000);
    expect(positionA.shares.toNumber()).to.eq(100_000_000);
    expect(positionB.shares.toNumber()).to.eq(50_000_000);

    await program.methods
      .withdraw(new anchor.BN(50_000_000))
      .accounts({
        owner: userA.publicKey,
        vault: vaultPda,
        vaultAuthority: vaultAuthorityPda,
        position: userAPosition,
        ownerTokenAccount: userAToken,
        vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([userA])
      .rpc();

    const vaultAfter = await program.account.vault.fetch(vaultPda);
    expect(vaultAfter.totalAssets.toNumber()).to.eq(200_000_000);
    expect(vaultAfter.totalShares.toNumber()).to.eq(100_000_000);
  });

  it("enforces pause flags", async () => {
    await program.methods
      .setPauseFlags(1)
      .accounts({ admin: admin.publicKey, vault: vaultPda })
      .rpc();

    try {
      await program.methods
        .deposit(new anchor.BN(1_000_000))
        .accounts({
          owner: userA.publicKey,
          vault: vaultPda,
          vaultAuthority: vaultAuthorityPda,
          position: userAPosition,
          ownerTokenAccount: userAToken,
          vaultTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([userA])
        .rpc();
      expect.fail("deposit should be paused");
    } catch (e) {
      expect(`${e}`).to.contain("Deposits are paused");
    }

    await program.methods
      .setPauseFlags(0)
      .accounts({ admin: admin.publicKey, vault: vaultPda })
      .rpc();
  });

  it("enforces role checks", async () => {
    try {
      await program.methods
        .setPauseFlags(2)
        .accounts({ admin: userA.publicKey, vault: vaultPda })
        .signers([userA])
        .rpc();
      expect.fail("non-admin should fail");
    } catch (e) {
      expect(`${e}`).to.contain("Unauthorized");
    }

    try {
      await program.methods
        .compound(new anchor.BN(1_000_000))
        .accounts({
          strategist: userA.publicKey,
          vault: vaultPda,
          strategistRewardTokenAccount: userAToken,
          vaultTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([userA])
        .rpc();
      expect.fail("non-strategist should fail");
    } catch (e) {
      expect(`${e}`).to.contain("Unauthorized");
    }
  });

  it("enforces caps", async () => {
    await program.methods
      .setCaps(new anchor.BN(210_000_000), new anchor.BN(200_000_000), new anchor.BN(5_000_000))
      .accounts({ admin: admin.publicKey, vault: vaultPda })
      .rpc();

    try {
      await program.methods
        .compound(new anchor.BN(6_000_000))
        .accounts({
          strategist: strategist.publicKey,
          vault: vaultPda,
          strategistRewardTokenAccount: strategistToken,
          vaultTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([strategist])
        .rpc();
      expect.fail("compound cap should fail");
    } catch (e) {
      expect(`${e}`).to.contain("Compound amount exceeds per-transaction cap");
    }
  });

  after(async () => {
    const tokenBal = await getAccount(provider.connection, vaultTokenAccount);
    expect(Number(tokenBal.amount)).to.greaterThan(0);
  });
});

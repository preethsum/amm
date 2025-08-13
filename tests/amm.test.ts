import * as anchor from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getMint,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("AMM Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.amm as anchor.Program<Amm>;

  // Mints
  let mintXOwner = anchor.web3.Keypair.generate();
  let mintYOwner = anchor.web3.Keypair.generate();
  let mintX: PublicKey;
  let mintY: PublicKey;

  let initializer = anchor.web3.Keypair.generate();
  let pool: PublicKey;
  let vaultX: PublicKey;
  let vaultY: PublicKey;
  let mintLp: PublicKey;
  before(async () => {
    // airdrop initialzer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        initializer.publicKey,
        LAMPORTS_PER_SOL
      )
    );

    // airdrop mint owners
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        mintXOwner.publicKey,
        LAMPORTS_PER_SOL
      )
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        mintYOwner.publicKey,
        LAMPORTS_PER_SOL
      )
    );

    // Create Mints
    mintX = await createMint(
      provider.connection,
      mintXOwner,
      mintXOwner.publicKey,
      mintXOwner.publicKey,
      9,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    mintY = await createMint(
      provider.connection,
      mintYOwner,
      mintYOwner.publicKey,
      mintYOwner.publicKey,
      9,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    // Get pool
    [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), mintX.toBuffer(), mintY.toBuffer()],
      program.programId
    );

    // Get vaults
    vaultX = getAssociatedTokenAddressSync(
      mintX,
      pool,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );
    vaultY = getAssociatedTokenAddressSync(
      mintY,
      pool,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );

    // get mint lp
    [mintLp] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), pool.toBuffer()],
      program.programId
    );
  });

  it("Initialze AMM", async () => {
    const swapFee = 10;
    try {
      await program.methods
        .initializePool(swapFee)
        .accountsPartial({
          initializer: initializer.publicKey,
          mintX: mintX,
          mintY: mintY,
          vaultX: vaultX,
          vaultY: vaultY,
          pool: pool,
          mintLp: mintLp,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          systemProgram: SYSTEM_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([initializer])
        .rpc();
    } catch (error) {
      console.log("---------- ERROR ERROR ----------");
      console.log(error);
      throw new error();
    }
  });
});

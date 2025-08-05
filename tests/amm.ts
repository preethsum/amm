import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { LiteSVM } from "litesvm";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { expect } from "chai";

describe("AMM Program", () => {
  let svm: LiteSVM;
  let program: Program<Amm>;
  let provider: anchor.AnchorProvider;
  let payer: Keypair;

  before(async () => {
    // Initialize LiteSVM
    svm = new LiteSVM();

    // ✅ Generate keypair using Solana's method
    payer = Keypair.generate();

    // Airdrop SOL to payer
    await svm.airdrop(payer.publicKey, 10 * LAMPORTS_PER_SOL);

    // Load your program
    const programKeypair = Keypair.generate(); // ✅ Correct way
    await svm.addProgram(programKeypair.publicKey, "./target/deploy/amm.so");

    // Setup provider and program
    provider = new anchor.AnchorProvider(
      svm.getConnection(),
      new anchor.Wallet(payer),
      { commitment: "confirmed" }
    );

    anchor.setProvider(provider);
    program = new Program(
      require("../target/idl/amm.json"),
      programKeypair.publicKey,
      provider
    );
  });

  it("should initialize AMM pool", async () => {
    // ✅ Generate keypair for pool account
    const poolAccount = Keypair.generate();

    const tx = await program.methods
      .initializePool()
      .accounts({
        pool: poolAccount.publicKey,
        authority: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([poolAccount])
      .rpc();

    expect(tx).to.be.a("string");
  });

  it("should create funded test account", async () => {
    // Helper function to create funded accounts
    const createFundedAccount = async (lamports: number = LAMPORTS_PER_SOL) => {
      const account = Keypair.generate();
      await svm.airdrop(account.publicKey, lamports);
      return account;
    };

    const testUser = await createFundedAccount(2 * LAMPORTS_PER_SOL);
    const balance = await svm.getConnection().getBalance(testUser.publicKey);

    expect(balance).to.equal(2 * LAMPORTS_PER_SOL);
  });
});

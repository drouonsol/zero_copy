import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Test01 } from "../target/types/test_01";

describe("test_01", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Test01 as Program<Test01>;
  const wallet = anchor.workspace.Test01.provider.wallet


  it("Create Account", async () => {
    // Add your test here.
    const tx = await program.methods.createAccount().rpc({
      skipPreflight: true
    });
    console.log("Your transaction signature", tx);
  });

  it("Is initialized!", async () => {
    const [StatePda] = await anchor.web3.PublicKey.findProgramAddress(
      [ Buffer.from("data_holder_drou_v10"), wallet.publicKey.toBuffer()],
      program.programId
    )
 
    // Add your test here.
    const tx = await program.methods.initialize().accounts({ dataHolder: StatePda }).rpc({
      skipPreflight: true
    });
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SwapApp } from "../target/types/swap_app";

describe("swap_app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SwapApp as Program<SwapApp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

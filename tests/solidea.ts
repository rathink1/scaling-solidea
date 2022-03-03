import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solidea } from "../target/types/solidea";

describe("solidea", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Solidea as Program<Solidea>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

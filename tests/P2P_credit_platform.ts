import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { P2pCreditPlatform } from "../target/types/p2p_credit_platform";

describe("P2P_credit_platform", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.P2pCreditPlatform as Program<P2pCreditPlatform>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

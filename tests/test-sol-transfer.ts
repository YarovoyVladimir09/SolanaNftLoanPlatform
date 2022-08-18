import * as anchor from "@project-serum/anchor";
// ** Comment this to use solpg imported IDL **
import {
  createKeypairFromFile,
} from './util';
import { P2pCreditPlatform } from "../target/types/P2P_credit_platform";


describe("token_transfer", async () => {
  

  const provider = anchor.AnchorProvider.env()
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);
  const program = anchor.workspace.P2pCreditPlatform as anchor.Program<P2pCreditPlatform>;


  it("Transfer!", async () => {

    // Testing constants

    const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const buyer: anchor.web3.Keypair = await createKeypairFromFile(
      "/home/vladimir/.config/solana/id2.json"
      );
    console.log(`Buyer public key: ${buyer.publicKey}`);
    // Transact with the "sell" function in our on-chain program
    
    await program.methods.solTransfer(
      new anchor.BN(saleAmount)
    )
    .accounts({
      ownerAuthority: wallet.publicKey,
      buyerAuthority: buyer.publicKey,
    })
    .signers([buyer])
    .rpc();
  });
});
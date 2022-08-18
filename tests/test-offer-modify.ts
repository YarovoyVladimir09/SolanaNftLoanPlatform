import * as anchor from "@project-serum/anchor";
// ** Comment this to use solpg imported IDL **
import {
  createKeypairFromFile,
} from './util';
import { P2pCreditPlatform } from "../target/types/p2p_credit_platform";



function shortKey(key: anchor.web3.PublicKey) {
  return key.toString().substring(0, 8);
}


describe("pdas", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.P2pCreditPlatform as anchor.Program<P2pCreditPlatform>;
  const wallet = provider.wallet as anchor.Wallet;

  async function derivePda(wallet: anchor.web3.PublicKey,
    creditor: anchor.web3.PublicKey,debtor: anchor.web3.PublicKey,
    time_mark: number) {
    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        wallet.toBuffer(),
        creditor.toBuffer(),
        Buffer.from("_"),
        debtor.toBuffer(),
        Buffer.from("_"),
        Buffer.from(time_mark.toString()),
      ],
      program.programId
    );
    return pda;
  }

  async function openOffer(
    creditor: anchor.web3.PublicKey,
    debtor: anchor.web3.PublicKey,
    time_mark: number,
    mint_account: string,
    money_count: number, 
    wallet: anchor.web3.Keypair
  ) {
    let pda = await derivePda(wallet.publicKey, creditor,
        debtor, time_mark);
    await program.methods.openOffer(creditor.toString(),
        debtor.toString(), new anchor.BN(time_mark), mint_account, money_count)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
  }

  async function closeOffer(
    creditor: anchor.web3.PublicKey,
    debtor: anchor.web3.PublicKey,
    time_mark: number,
    mint_account: string,
    money_count: number,
    wallet: anchor.web3.Keypair
  ) {

    console.log("--------------------------------------------------");
    let data;
    let pda = await derivePda(wallet.publicKey, creditor,
        debtor, time_mark);

    console.log(`Checking if account ${shortKey(pda)} 
    exists for creditor: ${shortKey(creditor)}
    and debtor: ${shortKey(debtor)}`);
    try {

      data = await program.account.ledger.fetch(pda);
      console.log("It does.");
    
    } catch (e) {
    
      console.log("It does NOT. Creating...");
      await openOffer(creditor, debtor,
       time_mark, mint_account, money_count, wallet);
      data = await program.account.ledger.fetch(pda);
    };

    console.log("Success.");
    console.log("Data:")
    // console.log(`    Color: ${data.color}   Balance: ${data.balance}`);
    // console.log(`Modifying balance of ${data.color} from ${data.balance} to ${newBalance}`);
    console.log(` Money back?: ${data.moneyback}`);

    await program.methods.closeOffer()
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    data = await program.account.ledger.fetch(pda);
    console.log("New Data:")
    console.log(` Money back?: ${data.moneyback}`);
    console.log("Success.");
  }


  it("An example of PDAs in action", async () => {

    const creditor: anchor.web3.Keypair = await createKeypairFromFile(
        "/home/vladimir/.config/solana/id2.json"
        );
        const debtor: anchor.web3.Keypair = await createKeypairFromFile(
            "/home/vladimir/.config/solana/id3.json"
            );
    

    //const testKeypair1 = await generateKeypair();
    await openOffer(creditor.publicKey,
    debtor.publicKey, 15000, "6K4cqGpWgGk89G7vcDmnufihyzS9sxGcvv3GXSeSv6sC",
    11.5, wallet.payer);

    await closeOffer(creditor.publicKey,
        debtor.publicKey, 15000, "6K4cqGpWgGk89G7vcDmnufihyzS9sxGcvv3GXSeSv6sC",
        11.5, wallet.payer)
    // await modifyLedger("blue", 2, testKeypair1);

    // const testKeypair2 = await generateKeypair();
    // await modifyLedger("red", 3, testKeypair2);
    // await modifyLedger("green", 3, testKeypair2);
  });
});

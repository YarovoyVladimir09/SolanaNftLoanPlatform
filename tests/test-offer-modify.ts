import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  createKeypairFromFile,
} from './util';
import { CreditPlatform } from "../target/types/credit_platform";



function shortKey(key: anchor.web3.PublicKey) {
  return key.toString().substring(0, 8);
}


describe("pdas", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  //const program = anchor.workspace.P2pCreditPlatform as anchor.Program<P2pCreditPlatform>;
  const program = anchor.workspace.CreditPlatform as Program<CreditPlatform>;
  //const programId = new anchor.web3.PublicKey('7nwV8W1EJrsJ2QFdPVuUMbXhWbpRpo82cVfzDiB9LJhc');
  const wallet = provider.wallet as anchor.Wallet;

  async function derivePda(
        creditor: anchor.web3.PublicKey,debtor: anchor.web3.PublicKey, mint_account: anchor.web3.PublicKey) {
    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        creditor.toBuffer(),
        mint_account.toBuffer(),
        debtor.toBuffer()
      ],
      program.programId
    );
    return pda;
  }

  async function openOffer(
    creditor: anchor.web3.PublicKey,
    debtor: anchor.web3.PublicKey,
    time_mark: number,
    mint_account: anchor.web3.PublicKey,
    money_count: number, 
    wallet: anchor.web3.Keypair
  ) {
    console.log("-----------------derive offer address---------------------------------");
    let pda = await derivePda(creditor,
        debtor, mint_account);
    console.log("-----------------open offer---------------------------------");
    await program.methods.openOffer(new anchor.BN(time_mark), new anchor.BN(money_count))
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
        creditor: creditor,
        debtor: debtor,
        mintAccount: mint_account
      })
      .signers([wallet])
      .rpc();
  }

  async function closeOffer(
    creditor: anchor.web3.PublicKey,
    debtor: anchor.web3.PublicKey,
    time_mark: number,
    mint_account: anchor.web3.PublicKey,
    money_count: number,
    wallet: anchor.web3.Keypair
  ) {

    console.log("--------------------------------------------------");
    let data;
    let pda = await derivePda(creditor,
        debtor, mint_account);

    console.log(`Checking if account ${(pda)} 
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
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
          "6K4cqGpWgGk89G7vcDmnufihyzS9sxGcvv3GXSeSv6sC"
        );
    

    //const testKeypair1 = await generateKeypair();
    // await openOffer(creditor.publicKey,
    // debtor.publicKey, 15000, mint,
    // 1150, wallet.payer);

    await closeOffer(creditor.publicKey,
        debtor.publicKey, 15000, mint,
        1150, wallet.payer)
    // await modifyLedger("blue", 2, testKeypair1);

    // const testKeypair2 = await generateKeypair();
    // await modifyLedger("red", 3, testKeypair2);
    // await modifyLedger("green", 3, testKeypair2);
  });
});

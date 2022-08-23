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
  const program = anchor.workspace.CreditPlatform as Program<CreditPlatform>;
  const wallet = provider.wallet as anchor.Wallet;

  async function deriveLedger(
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

  async function deriveBook(
    wallet: anchor.web3.PublicKey, loan_participater: anchor.web3.PublicKey) {
let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
  [
    wallet.toBuffer(),
    Buffer.from("_book_"),
    loan_participater.toBuffer()
  ],
  program.programId
);
return pda;
}

  async function openBook(
    participater: anchor.web3.PublicKey,
    ledger_account: anchor.web3.PublicKey,
    wallet: anchor.web3.Keypair
  ) {
    console.log("-----------------derive book address---------------------------------");
    let book = await deriveBook(wallet.publicKey,
      participater);
    console.log("-----------------open book---------------------------------");
    await program.methods.openBook()
      .accounts({
        bookAccount: book,
        wallet: wallet.publicKey,
        loanParticipater: participater,
        ledgerAccount: ledger_account
      })
      .signers([wallet])
      .rpc();
  }

  async function modifyBook(
    creditor: anchor.web3.PublicKey,
    debtor: anchor.web3.PublicKey,
    mint_account: anchor.web3.PublicKey,
    wallet: anchor.web3.Keypair,
  ) {

    console.log("------------------Derive ledger account--------------------------------");
    let data_creditor;
    let data_debtor;
    let ledger = await deriveLedger(creditor,
        debtor, mint_account);

    console.log("Success.");

    let book1 = await deriveBook(wallet.publicKey,
      creditor);

    let book2 = await deriveBook(wallet.publicKey,
      debtor);

    console.log(`Checking if account ${(book1)} 
    exists for account: ${shortKey(creditor)}`);
    try {

      data_creditor = await program.account.ledgerBook.fetch(book1);
      console.log("It does.");
    
    } catch (e) {
    
      console.log("It does NOT. Creating...");
      await openBook(creditor,ledger, wallet);
      data_creditor = await program.account.ledgerBook.fetch(book1);
    };

    console.log(`Checking if account ${(book2)} 
    exists for account: ${shortKey(debtor)}`);
    try {

      data_debtor = await program.account.ledgerBook.fetch(book2);
      console.log("It does.");
    
    } catch (e) {
    
      console.log("It does NOT. Creating...");
      await openBook(debtor,ledger, wallet);
      data_debtor = await program.account.ledgerBook.fetch(book2);
    };

    console.log(`Creditor Book[0]:  ${data_creditor.data[0].toString()}`)
    console.log(`Debtor Book[0]:  ${data_debtor.data[0].toString()}`)
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

    await modifyBook(creditor.publicKey,
          debtor.publicKey,mint, wallet.payer);
  });
});

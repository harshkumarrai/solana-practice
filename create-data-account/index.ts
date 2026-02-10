//create a data account

import { Connection, Keypair, PublicKey, SystemProgram, Transaction, type SignatureStatusConfig } from "@solana/web3.js";
    const conn=await new Connection("http://127.0.0.1:8899");

async function main() {
    const kp=new Keypair();
    const data_account=new Keypair();
    //airdrop karna hoga + connect tp local blockchain + 
    const balance=await conn.getBalance(kp.publicKey);
    console.log(balance);
   const s= await conn.requestAirdrop(kp.publicKey, 1000_000_0000);
await conn.confirmTransaction(s);
const baslance=await conn.getBalance(kp.publicKey);
    // console.log(balance);
    console.log(baslance);
    const gg=kp.publicKey;
   const instruction= await SystemProgram.createAccount({
    fromPubkey: kp.publicKey,
    /** Public key of the created account */
    newAccountPubkey:data_account.publicKey,
    /** Amount of lamports to transfer to the created account */
    lamports: 1000_000,
    /** Amount of space in bytes to allocate to the created account */
    space: 3,
    /** Public key of the program to assign as the owner of the created account */
    programId: kp.publicKey,

});

    const tx=new Transaction().add(instruction);
    tx.feePayer=kp.publicKey;
    tx.recentBlockhash=(await conn.getLatestBlockhash()).blockhash;
    tx.sign(kp)
    await conn.sendTransaction(tx,[kp,data_account]);
    //the acc u are creating on solana has to sign the transaction
    console.log(data_account.publicKey.toBase58());
}   
main();
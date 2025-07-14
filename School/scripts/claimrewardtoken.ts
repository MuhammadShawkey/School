import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

import fs from "fs";

// Destructure the required components from `anchor`
const { Program, AnchorProvider, Wallet, BN, web3 } = anchor;

// Set the connection to the Solana network
const connection = new Connection("http://api.devnet.solana.com", "confirmed");

const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("/home/mohamed/Shawkey.json", "utf8")))
  );

const wallet = new Wallet(walletKeypair);
const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
anchor.setProvider(provider);

// Ensure the correct program ID and IDL are used
const idlString = fs.readFileSync("target/idl/elmadrasa.json", "utf8");
const idl = JSON.parse(idlString) as anchor.Idl;
const program = new Program(idl, provider);
const goodstudent = provider.wallet;


const [studentsPda] =  anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("students")],
  program.programId
);

const main = async () => {

  const student_name = "Shawkey";
  const class_index = 0 ;
  const signer_token_account = new PublicKey('E7pm4Cf5NyT41gGiusXiMkyeBDfRQBFdrcWSosbXTucb');
const tokenAddress = new PublicKey('f5YDUna6in1FBYpPBWgRN8Xw8HDwKu8n9Ebfoxb7nyk');
const treasurytokenaccount = new PublicKey('J1x9SG2CsYiepahDBrwrCMFPeFj1TEjPiaeGyLQfqogv');
const treasurydash = new PublicKey('7H9uarEDHBYYf7toZ2uYGNnL4UcAymFYaHtcZ7vGDWKv');


  try {
    const txHash = await program.methods.claimrewardtoken(student_name , class_index)
      .accounts({
        signer : goodstudent.publicKey,
        signerTokenAccount: signer_token_account,
        treasuryTokenAccount: treasurytokenaccount,
        treasurydash: treasurydash,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMint: tokenAddress,
        students: studentsPda,
      })
      .rpc();

    console.log("Transaction hash:", txHash);

    // Optional: Wait for confirmation
    await connection.confirmTransaction(txHash);
  } catch (err) {
    console.error("Error calling deposit_for_erc20 function:", err);

    if (err.logs) {
      err.logs.forEach(log => console.error(log));
    }
  }
}

// Example usage:
main().catch(console.error);

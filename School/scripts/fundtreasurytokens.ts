import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import fs from "fs";

// Destructure the required components from `anchor`
const { Program, AnchorProvider, Wallet, BN, web3 } = anchor;

// Set the connection to the Solana network
const connection = new Connection("http://api.devnet.solana.com", "confirmed");

const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("/home/mohamed/id.json", "utf8")))
  );

const wallet = new Wallet(walletKeypair);
const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
anchor.setProvider(provider);

// Ensure the correct program ID and IDL are used
const idlString = fs.readFileSync("target/idl/elmadrasa.json", "utf8");
const idl = JSON.parse(idlString) as anchor.Idl;
const program = new Program(idl, provider);

const main = async () => {

const tokenAddress = new PublicKey('f5YDUna6in1FBYpPBWgRN8Xw8HDwKu8n9Ebfoxb7nyk');
const senderata = new PublicKey('DFrT12edcBBAGUKVy5mmmz8Kme4DtTSJzy3cgkvGMVQn');
const treasurytokenaccountreceiever = new PublicKey('J1x9SG2CsYiepahDBrwrCMFPeFj1TEjPiaeGyLQfqogv');
const owneraddress = new PublicKey('FwAu9vzoZwxkYAVCQytdKFcGMpUhdxEkPZFZxscmh6uZ');
const amount_fund = new BN (1_000_000_000) ;
  try {
    const txHash = await program.methods.fundtreasurytokens(
   amount_fund
    )
      .accounts({
        signer : wallet.publicKey,
        senderata : senderata,
        treasuryTokenAccountReciever : treasurytokenaccountreceiever,
        owneraddress: owneraddress ,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMint: tokenAddress
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

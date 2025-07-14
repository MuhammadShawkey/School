  import * as anchor from '@coral-xyz/anchor';
  import { Keypair, PublicKey } from '@solana/web3.js';
  import { AnchorProvider, Wallet, Idl , BN } from '@coral-xyz/anchor';
  import fs from 'fs';
  
  // Setup connection and wallet
  const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"), "confirmed");

  const walletKeypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("/home/mohamed/id.json", "utf8")))
  );
  const wallet = new Wallet(walletKeypair);
  const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
  anchor.setProvider(provider);

  // Load IDL and program
  const idlString = fs.readFileSync("target/idl/elmadrasa.json", "utf8");
  const idl = JSON.parse(idlString) as Idl;
  const program = new anchor.Program(idl, provider);

  
  // ---------------------- Main Script ----------------------

  const [treasurydash] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasurydash")],
      program.programId
    );
  const main = async () => {
    // The logic inside here will be able to use `await`
    
    const amount_in_lamports = new anchor.BN(1000000000) ;
    const ceo = provider.wallet;
    
    const tx = await program.methods
      .fundtreasurynative(amount_in_lamports)
      .accounts({
        signer: ceo.publicKey,
        treasurydash: treasurydash,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    }

    main().catch(console.error);



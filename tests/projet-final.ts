import { ProjetFinal } from './../target/types/projet_final';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Keypair, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("projet-final", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ProjetFinal as Program<ProjetFinal>;
  const connection = new Connection('http://localhost:8899', 'confirmed');
  
  const authority =Keypair.generate();
  const wallet = Keypair.generate();

  

  it("Is initialized!", async () => {

    const lamports = 10 * LAMPORTS_PER_SOL;
    const tx = await connection.requestAirdrop(wallet.publicKey, lamports);
    await connection.confirmTransaction(tx);
    
    

    const balance = await connection.getBalance(wallet.publicKey);
    console.log("Wallet balance : ", balance);

   
    
    const tx3 = await connection.requestAirdrop(authority.publicKey, lamports);
    await connection.confirmTransaction(tx3);
    const balance2 = await connection.getBalance(wallet.publicKey);
    console.log("Signer balance : ", balance2);


    const mapData = Buffer.from( [1, 1, 1, 1, 1, 0, 0, 1, 1,0, 0, 1, 1, 1,1 , 1]); 
    const width = 4;
    const height = 4;

    // Find the PDA for the game account
    const [gamePDA, bump] = await PublicKey.findProgramAddress(
    [authority.publicKey.toBuffer()],
    program.programId);

    const tx2 = await program.rpc.initialize(width, height, mapData, {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });


    // Log mapData with line breaks at each width
    console.log("Map data:");
    for (let i = 0; i < height; i++) {
      let line = "";
      for (let j = 0; j < width; j++) {
        line += mapData[i * width + j] + " ";
      }
      console.log(line.trim());
    }

    
    
    console.log("Ajout d'une caisse en 1-2")
    const tx4 = await program.rpc.addItem(2,1,2,  {
      accounts: {
        game: gamePDA,
        signer: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });

    const updatedGame = await program.account.gameState.fetch(gamePDA);
    const updatedMapData = updatedGame.mapData;
    // Log mapData with line breaks at each width
    console.log("Map data:");
    for (let i = 0; i < height; i++) {
      let line = "";
      for (let j = 0; j < width; j++) {
        line += updatedMapData[i * width + j] + " ";
      }
      console.log(line.trim());
    }


  });
});

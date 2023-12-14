import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowDonate } from "../target/types/escrow_donate";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed"
  )
  return signature
}

describe("escrow-donate", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as NodeWallet;

  const program = anchor.workspace.EscrowDonate as Program<EscrowDonate>;

  const [escrowDonate] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow-donate"), 
      anchor.getProvider().publicKey.toBytes()
    ], 
    program.programId
  );

  let mint: PublicKey;
  let makerATA: PublicKey;

  const donator = anchor.web3.Keypair.generate();
  
  // Request an airdrop for our donator.
  it("Airdrop", async () => {
      const tx = await anchor.getProvider().connection.requestAirdrop(donator.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL).then(confirmTx);

      console.log("Airdrop from provider successful! Your transaction signature", tx);
  });

  it("Donate Escrow is initialized!", async () => {
    // Add your test here.
    mint = await createMint(provider.connection, wallet.payer, wallet.publicKey, null, 6);
    makerATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, mint, wallet.publicKey)).address;

    //const ata = getAssociatedTokenAddressSync(mint, escrowDonate)

    const vaultATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, mint, escrowDonate, true)).address;
    console.log("Vault ATA: ", vaultATA.toBase58());
    
    const tx = await program.methods
    .make(new BN(1_000_000))
    .accounts({
      signer: wallet.publicKey,
      escrow: escrowDonate,
      vaultAta: vaultATA,
      makerAta: makerATA,
      mint: mint,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Make a donation!", async () => {
    const donatorATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, mint, donator.publicKey)).address;
    console.log("Donator ATA: ", donatorATA.toBase58());
    const vaultATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, mint, escrowDonate, true)).address;
    console.log("Vault ATA: ", vaultATA.toBase58());
    const rewardsMint = await createMint(provider.connection, wallet.payer, wallet.publicKey, wallet.publicKey, 6);
    console.log("Rewards Mint: ", rewardsMint.toBase58());

    const mintTx = await mintTo(provider.connection, wallet.payer, mint, donatorATA, wallet.payer, 100000000).then(confirmTx);
    console.log("Minted 100_000_000 tokens to donator: ", mintTx);

    const DonateTx = await program.methods.donate(new BN(1_000_000))
    .accounts({
      signer: donator.publicKey,
      owner: wallet.publicKey,
      escrow: escrowDonate,
      vaultAta: vaultATA,
      donatorAta: donatorATA,
      makerAta: makerATA,
      mint: mint,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      rewardsMint: rewardsMint,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([donator])
    .rpc();

    const evaluateTx = await program.methods.checkDonations()
    .accounts({
      signer: donator.publicKey,
      owner: wallet.publicKey,
      escrow: escrowDonate,
      vaultAta: vaultATA,
      donatorAta: donatorATA,
      makerAta: makerATA,
      mint: mint,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      rewardsMint: rewardsMint,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([donator])
    .rpc();
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BibliotecaSolana } from "../target/types/biblioteca_solana";
async function main() {
 const provider = anchor.AnchorProvider.env();
 anchor.setProvider(provider);
 const program = anchor.workspace.BibliotecaSolana as Program<BibliotecaSolana>;
 
 const [perfilPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("perfil_cine"), provider.wallet.publicKey.toBuffer()],
  program.programId
 );
 try {
  console.log("Agregando película...");
  
  const tx = await program.methods
   .agregarPelicula(
    "Batman: El Caballero de la Noche", // Título
    "Accion",                           // Género
    5                                   // Calificación
    )
   .accounts({
    perfil: perfilPDA,
    usuario: provider.wallet.publicKey,
   }) 
   .rpc();
  console.log("¡PELÍCULA GUARDADA! Tarea terminada.");
  console.log("Firma de la transacción:", tx); 
 } catch (error) {
  console.error("Error al guardar:", error);
 }
}
main();

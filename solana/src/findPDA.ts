// import {} from "@solana/web3.js"
import { getAssociatedTokenAddressSync } from "@solana/spl-token"
import { PublicKey } from "@solana/web3.js"

const something = getAssociatedTokenAddressSync(new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), new PublicKey("7atNhkWdAgQSJRauihSnsP1oaSAmoQu8z3vDgQyXVADD"))
 

console.log(something.toBase58());


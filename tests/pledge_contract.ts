import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PledgeContract } from "../target/types/pledge_contract";
import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY
} from '@solana/web3.js';
import { deserialize, Schema } from 'borsh';
import { BN } from "bn.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID, associatedAddress } from "@project-serum/anchor/dist/cjs/utils/token";

describe("pledge_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PledgeContract as Program<PledgeContract>;

  it("It initialize!", async () => {
   
  });
});

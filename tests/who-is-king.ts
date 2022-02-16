import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { WhoIsKing } from '../target/types/who_is_king';

describe('who-is-king', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.WhoIsKing as Program<WhoIsKing>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.guessKing("0xdeep", {});
    console.log("Your transaction signature", tx);
  });
});

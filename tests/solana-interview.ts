import * as anchor from '@coral-xyz/anchor';
import { Program, web3, BN } from '@coral-xyz/anchor';
import { SolanaInterview } from '../target/types/solana_interview';
import { assert } from 'chai';

describe('solana-interview', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaInterview as Program<SolanaInterview>;
  const id = 35; //would have used a random generator
  // Find PDA for the account
  const [taskAccount, taskBump] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('task'),
      program.provider.publicKey.toBuffer(),
      new BN(id).toBuffer('le', 8),
    ],
    program.programId
  );
  it('Task Is initialized!', async () => {
    // Add your test here.
    console.log('task account:', taskAccount);
    let taskTitle = 'Wake up';
    let taskDescription = `
    I wake up blassasassasss sasjajajdksjvdvsjdksdjbfskjdvbsdbfkshbfskfbksfkskfhbskfs
    `;
    const txSig = await program.methods
      .createTask(new BN(id), taskTitle, taskDescription)
      .accounts({
        taskAccount: taskAccount,
        owner: program.provider.publicKey,
      })
      .rpc();
    // console.log('Your transaction signature on creation', txSig);
    // Fetch account
    const task = await program.account.task.fetch(taskAccount);
    // console.log('Created Task:', task);
    assert(txSig);
  });
  it('Task Is updated!', async () => {
    // Add your test here.
    console.log('taskAccount:', taskAccount);
    let taskTitle = 'Wake me up';
    let taskDescription = `
    I wake up blassasassasss bfkshbfskfbksfkskfhbskfs
    `;
    const txSig = await program.methods
      .updateTask(new BN(id), taskTitle, taskDescription, taskBump)
      .accounts({
        taskAccount: taskAccount,
        owner: program.provider.publicKey,
      })
      .rpc();
    // console.log('Your transaction signature on update:', txSig); // Fetch account
    const task = await program.account.task.fetch(taskAccount);
    // console.log('Updated Task:', task);
    assert.equal(task.title, taskTitle);
    assert.equal(task.description, taskDescription);
    assert(txSig);
  });
  it('Task Is completed!', async () => {
    // Add your test here.

    const txSig = await program.methods
      .completeTask(new BN(id), taskBump)
      .accounts({
        taskAccount: taskAccount,
        owner: program.provider.publicKey,
      })
      .rpc();
    // console.log('Your transaction signature on completion:', txSig); // Fetch account
    const task = await program.account.task.fetch(taskAccount);
    // console.log('Completed Task:', task);
    assert.equal(task.completed, true);

    assert(txSig);
  });
});

import { spawn } from 'child_process';
import { join } from 'path';
import { CONTRACT_ADDRESS } from '../src/utils/configs';

if (!CONTRACT_ADDRESS) {
  console.error('Please provide an address for CONTRACT_ADDRESS in /frontend/src/utils/configs.ts');
  process.exit(1);
}

const command = 'cargo';
const args = [
  'pbc',
  'abi',
  'codegen',
  `--contract=${CONTRACT_ADDRESS}`,
  '--ts',
  join(__dirname, '../src/utils/abi.ts')
];

const child = spawn(command, args, {
  stdio: 'inherit',
  shell: true
});

child.on('close', (code) => {
  if (code !== 0) {
    console.error(`Process exited with code ${code}`);
    process.exit(code);
  }
  console.log('ABI generation completed successfully');
});

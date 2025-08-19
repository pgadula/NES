
import wasmInit, { WebEmu } from "../pkg/web_emulator"
await wasmInit();

const STEPS_PER_FRAME = 30000;
const WIDTH = 256;
const HEIGHT = 240;

const screen = document.getElementById('screen') as HTMLCanvasElement;
const pause_btn = document.getElementById('pause') as HTMLButtonElement;
const step_btn = document.getElementById('step') as HTMLButtonElement;
const cpu_registers_el: HTMLUListElement = document.getElementById('cpu_registers') as HTMLUListElement;
const memory_el = document.getElementById('memory') as HTMLDivElement;
const nametable_el = document.getElementById('nametable') as HTMLDivElement;
const memoryPtr_el = document.getElementById('ptr') as HTMLInputElement;


const ctx = screen.getContext('2d');
const imageData = ctx.createImageData(WIDTH, HEIGHT);
let emu = null;
let running = false;
let lastRenderTime = 0;

async function start() {
  function render(ms) {
    if (emu) {
      if (running) {
        for (let i = 0; i < STEPS_PER_FRAME; i++) {
          emu.step();
        }
      }

      const selected_nametable = +(document.querySelector('input[name="nametable"]:checked') as HTMLInputElement)?.value;

      memoryDump(nametable_el, emu.nametable(selected_nametable), 64, 32)
      memoryDump(memory_el, emu.ramDump(+memoryPtr_el.value), +memoryPtr_el.value); 
      renderCpuRegisters(cpu_registers_el, Array.from(emu.cpuRegisters()));


      if (!lastRenderTime || ms - lastRenderTime >= 16) {
        lastRenderTime = ms;

        const framebuffer = emu.getFramebuffer();
        const buf = new Uint32Array(imageData.data.buffer);
        buf.set(framebuffer);
        ctx.putImageData(imageData, 0, 0);
      }
    }
    requestAnimationFrame(render);
  }

  document.getElementById('fileInput').addEventListener('change', async (event) => {
    const file = (event.target as any).files[0];
    if (!file) return;

    const bytes = await file.arrayBuffer();

    if (typeof WebEmu.loadCartridge === 'function') {
      emu = WebEmu.loadCartridge(new Uint8Array(bytes));
      running = true;

      console.log("Cartridge loaded and emulator running!");
    } else {
      console.error("loadCartridge is not defined yet.");
    }
  });

  requestAnimationFrame(render);
}

start();

pause_btn.addEventListener('click', () => {
  running = !running;
  pause_btn.textContent = running ? "PAUSE" : 'START';
})
step_btn.addEventListener('click', () => {
  emu.step();
})

function renderCpuRegisters(
  root: HTMLElement,
  [pc, p, a, x, y, sp]: number[]
) {
  const hex = (value: number, digits = 2) =>
    value.toString(16).toUpperCase().padStart(digits, '0');

  const flagsArray = [
    { name: 'N', bit: 7 }, // Negative
    { name: 'V', bit: 6 }, // Overflow
    { name: '-', bit: 5 }, // Unused
    { name: 'B', bit: 4 }, // Break
    { name: 'D', bit: 3 }, // Decimal
    { name: 'I', bit: 2 }, // Interrupt Disable
    { name: 'Z', bit: 1 }, // Zero
    { name: 'C', bit: 0 }, // Carry
  ];

  // Generate first row (N, V, -, B)
  const firstRow = flagsArray
    .slice(0, 4)
    .map(f => `${f.name}:${(p >> f.bit) & 1}`)
    .join(' ');

  // Generate second row (D, I, Z, C)
  const secondRow = flagsArray
    .slice(4)
    .map(f => `${f.name}:${(p >> f.bit) & 1}`)
    .join(' ');

  root.innerHTML = `
  <li class='item'>PC: 0x${hex(pc, 4)}</li>
  <li class='item'>&nbsp;A: 0x${hex(a)}</li>
  <li class='item'>&nbsp;X: 0x${hex(x)}</li>
  <li class='item'>&nbsp;Y: 0x${hex(y)}</li>
  <li class='item'>SP: 0x${hex(sp)}</li>
  <li class='item'>&nbsp;P: 0x${hex(p)}</li>
  <li class='item'>${firstRow}<br>${secondRow}</li>
`;
}

function memoryDump(root: HTMLElement, memoryDump: Uint8Array, ptr: number, offset = 16) {
  const lines = [];

  for (let i = 0; i < memoryDump.length; i += offset) {
    const chunk = memoryDump.slice(i, i + offset);

    const hexBytes = Array.from(chunk)
      .map(b => {
        const hue = Math.floor((b / 255) * 240); 
        const color = `hsl(${hue}, 80%, 60%)`;

        return `<span style="color:${color}">${b.toString(16).padStart(2, '0').toUpperCase()}</span>`;
      })
      .join(' ');

    const ascii = Array.from(chunk)
      .map(b => {
        const char = String.fromCharCode(b);
        let displayChar = b >= 32 && b < 127 ? char : '.';

        // Assign color based on character type
        let color = '#AAAAAA'; // default gray
        if (/[A-Z]/.test(displayChar)) color = '#FF6B6B'; // red for uppercase
        else if (/[a-z]/.test(displayChar)) color = '#4ECDC4'; // cyan for lowercase
        else if (/[0-9]/.test(displayChar)) color = '#FFD93D'; // yellow for numbers
        else if (displayChar === '.') color = '#999999'; // dull gray for non-printable

        return `<span style="color:${color}">${displayChar}</span>`;
      })
      .join('');
    const address = (ptr + (i * offset)).toString(16).padStart(4, '0').toUpperCase();
    lines.push(`${address}: ${hexBytes.padEnd(47)}  ${ascii}`);
  }

  root.innerHTML = `<pre>${lines.join('\n')}</pre>`;
}
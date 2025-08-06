
import wasmInit, { WebEmu } from "../pkg/web_emulator"
await wasmInit();

const STEPS_PER_FRAME = 30000;
const WIDTH = 256;
const HEIGHT = 240;
const screen = document.getElementById('screen') as HTMLCanvasElement;
const pause_btn = document.getElementById('pause') as HTMLButtonElement;
const step_btn = document.getElementById('step') as HTMLButtonElement;
const cpu_registers_el: HTMLUListElement = document.getElementById('cpu_registers') as HTMLUListElement;

const ctx = screen.getContext('2d');
const imageData = ctx.createImageData(WIDTH, HEIGHT);
let emu = null;
let running = false;
let lastRenderTime = 0;

async function start() {
  function render(ms) {
    if (emu) {
      if(running){
        for (let i = 0; i < STEPS_PER_FRAME; i++) {
          emu.step();
        }
      }

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

pause_btn.addEventListener('click', ()=>{
  running = !running;
  pause_btn.textContent =  running ? "PAUSE" : 'START';
})
step_btn.addEventListener('click', ()=>{
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
  <li>PC: 0x${hex(pc, 4)}</li>
  <li>&nbsp;A: 0x${hex(a)}</li>
  <li>&nbsp;X: 0x${hex(x)}</li>
  <li>&nbsp;Y: 0x${hex(y)}</li>
  <li>SP: 0x${hex(sp)}</li>
  <li>&nbsp;P: 0x${hex(p)}</li>
  <li>${firstRow}<br>${secondRow}</li>
`;
}
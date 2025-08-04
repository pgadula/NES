
import wasmInit, { WebEmu } from "../pkg/web_emulator"
await wasmInit();

const STEPS_PER_FRAME = 30000;
const WIDTH = 256;
const HEIGHT = 240;
const screen = document.getElementById('screen') as HTMLCanvasElement;
const cpu_registers_el: HTMLUListElement = document.getElementById('cpu_registers') as HTMLUListElement;

const ctx = screen.getContext('2d');
const imageData = ctx.createImageData(WIDTH, HEIGHT); // RGBA buffer
let emu = null;
let running = false;
let lastRenderTime = 0;

async function start() {

  function render(ms) {
    if (running && emu) {
      for (let i = 0; i < STEPS_PER_FRAME; i++) {
        emu.step();
      }
      let cpu_registers = new Uint16Array(emu.cpuRegisters());
      cpu_registers_el.innerHTML = Array.from(cpu_registers)
        .map(b => `<li>0x${b.toString(16).toUpperCase().padStart(4, '0')}</li>`)
        .join('');

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
      emu = WebEmu.loadCartridge(new Uint8Array(bytes)); // pass Uint8Array
      running = true; // start emulator
      console.log("Cartridge loaded and emulator running!");
    } else {
      console.error("loadCartridge is not defined yet.");
    }
  });

  requestAnimationFrame(render);
}

start();
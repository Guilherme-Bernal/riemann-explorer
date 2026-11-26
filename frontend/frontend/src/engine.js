// Motor de cálculo: usa o Rust compilado para WebAssembly. Se o WASM não estiver
// disponível (ex.: rodando sem build do wasm), cai para a API Rust local em /api.

let wasm = null
let mode = 'none'

const loaders = import.meta.glob('./wasm/riemann.js')

export async function initEngine() {
  const load = loaders['./wasm/riemann.js']
  if (load) {
    try {
      const m = await load()
      await m.default()
      wasm = m
      mode = 'wasm'
      return mode
    } catch (e) {
      console.warn('WASM indisponível, tentando a API', e)
    }
  }
  try {
    const r = await fetch('/api/zeros?to=20')
    if (r.ok) mode = 'api'
  } catch (e) {
    console.warn('API indisponível', e)
  }
  return mode
}

export const engineMode = () => mode

const flat = rows => Float64Array.from(rows.flat())
const get = async url => {
  const r = await fetch(url)
  if (!r.ok) throw new Error(`${url}: ${r.status}`)
  return r.json()
}

export async function zCurve(from, to, step) {
  if (wasm) return wasm.z_curve(from, to, step)
  return flat((await get(`/api/z?from=${from}&to=${to}&step=${step}`)).points)
}

export async function findZeros(to) {
  if (wasm) return { zeros: Array.from(wasm.find_zeros(to)), expected: wasm.zero_count_expected(to) }
  const j = await get(`/api/zeros?to=${to}`)
  return { zeros: j.zeros, expected: j.expected }
}

export async function heatmap(tMax, w, h) {
  if (wasm) return wasm.heatmap(tMax, w, h)
  return Float64Array.from((await get(`/api/heatmap?t_max=${tMax}&w=${w}&h=${h}`)).log10_abs)
}

export async function primesCurve(max, samples) {
  if (wasm) return wasm.primes_curve(max, samples)
  return flat((await get(`/api/primes?max=${max}&samples=${samples}`)).points)
}

export async function psiCurve(xMax, samples, zeros, k) {
  if (wasm) return wasm.psi_curve(xMax, samples, Float64Array.from(zeros), k)
  return flat((await get(`/api/psi?x=${xMax}&k=${k}&samples=${samples}`)).points)
}
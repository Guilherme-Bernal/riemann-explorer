// Gráficos em canvas no estilo do caderno: papel, tinta e vermelho.

export const COR = { tinta: '#1b1a17', vermelho: '#b3261e', apagado: '#8a8070', linha: '#d8cfbd', papel: '#f3ede1', azul: '#2f5d8a' }

export function prepare(cv) {
  const dpr = window.devicePixelRatio || 1
  const W = cv.clientWidth, H = cv.clientHeight
  cv.width = W * dpr
  cv.height = H * dpr
  const ctx = cv.getContext('2d')
  ctx.scale(dpr, dpr)
  ctx.clearRect(0, 0, W, H)
  return { ctx, W, H }
}

const fmt = (v, locale) => Math.abs(v) >= 1e4 ? v.toLocaleString(locale, { notation: 'compact' }) : (Math.round(v * 100) / 100 + 0).toLocaleString(locale)

function ticks(min, max, n = 6) {
  const raw = (max - min) / n
  const mag = 10 ** Math.floor(Math.log10(raw))
  const step = [1, 2, 5, 10].map(m => m * mag).find(s => s >= raw)
  const out = []
  for (let v = Math.ceil(min / step) * step; v <= max + 1e-9; v += step) out.push(v)
  return out
}

// series: [{ data: [[x,y],...], color, width, dash, step }]
export function lineChart(cv, series, o = {}) {
  const { ctx, W, H } = prepare(cv)
  const pad = { l: 56, r: 34, t: 14, b: 28 }
  const all = series.flatMap(s => s.data)
  const xmin = o.xmin ?? Math.min(...all.map(p => p[0]))
  const xmax = o.xmax ?? Math.max(...all.map(p => p[0]))
  let ymin = o.ymin ?? Math.min(...all.map(p => p[1]))
  let ymax = o.ymax ?? Math.max(...all.map(p => p[1]))
  if (o.symmetric) { const m = Math.max(Math.abs(ymin), Math.abs(ymax)); ymin = -m; ymax = m }
  const yr = (ymax - ymin) || 1
  ymin -= yr * 0.04; ymax += yr * 0.04
  const x = v => pad.l + (v - xmin) / (xmax - xmin) * (W - pad.l - pad.r)
  const y = v => pad.t + (ymax - v) / (ymax - ymin) * (H - pad.t - pad.b)

  ctx.font = '11px JetBrains Mono, monospace'
  ctx.fillStyle = COR.apagado
  ctx.strokeStyle = COR.linha
  ctx.lineWidth = 1
  ctx.setLineDash([2, 4])
  for (const v of ticks(xmin, xmax)) {
    ctx.beginPath(); ctx.moveTo(x(v), pad.t); ctx.lineTo(x(v), H - pad.b); ctx.stroke()
    ctx.fillText(fmt(v, o.locale), x(v) - 10, H - 8)
  }
  for (const v of ticks(ymin, ymax, 4)) {
    ctx.beginPath(); ctx.moveTo(pad.l, y(v)); ctx.lineTo(W - pad.r, y(v)); ctx.stroke()
    ctx.fillText(fmt(v, o.locale), 4, y(v) + 4)
  }
  ctx.setLineDash([])
  if (ymin < 0 && ymax > 0) {
    ctx.strokeStyle = COR.tinta
    ctx.beginPath(); ctx.moveTo(pad.l, y(0)); ctx.lineTo(W - pad.r, y(0)); ctx.stroke()
  }
  for (const s of series) {
    ctx.strokeStyle = s.color
    ctx.lineWidth = s.width ?? 1.5
    ctx.setLineDash(s.dash ?? [])
    ctx.beginPath()
    s.data.forEach((p, i) => {
      if (i === 0) return ctx.moveTo(x(p[0]), y(p[1]))
      if (s.step) ctx.lineTo(x(p[0]), y(s.data[i - 1][1]))
      ctx.lineTo(x(p[0]), y(p[1]))
    })
    ctx.stroke()
  }
  ctx.setLineDash([])
  if (o.dots) {
    ctx.fillStyle = COR.vermelho
    for (const d of o.dots) { ctx.beginPath(); ctx.arc(x(d), y(0), 4, 0, Math.PI * 2); ctx.fill() }
  }
  if (o.cursor != null) {
    ctx.strokeStyle = COR.vermelho; ctx.lineWidth = 1
    ctx.beginPath(); ctx.moveTo(x(o.cursor), pad.t); ctx.lineTo(x(o.cursor), H - pad.b); ctx.stroke()
  }
  return { toX: px => xmin + (px - pad.l) / (W - pad.l - pad.r) * (xmax - xmin) }
}

// Mapa de calor de log10|zeta|: sigma na horizontal, t na vertical (crescendo para cima).
export function heatChart(cv, vals, w, h, o) {
  const { ctx, W, H } = prepare(cv)
  const pad = { l: 44, r: 10, t: 14, b: 30 }
  const img = document.createElement('canvas')
  img.width = w; img.height = h
  const ic = img.getContext('2d')
  const data = ic.createImageData(w, h)
  const lo = -1.3, hi = 0.9
  const a = [27, 26, 23], b = [179, 38, 30], c = [243, 237, 225]
  for (let j = 0; j < h; j++) {
    for (let i = 0; i < w; i++) {
      const v = vals[j * w + i]
      const n = Math.min(1, Math.max(0, (v - lo) / (hi - lo)))
      const [p, q, f] = n < 0.45 ? [a, b, n / 0.45] : [b, c, (n - 0.45) / 0.55]
      const k = ((h - 1 - j) * w + i) * 4
      for (let ch = 0; ch < 3; ch++) data.data[k + ch] = p[ch] + (q[ch] - p[ch]) * f
      data.data[k + 3] = 255
    }
  }
  ic.putImageData(data, 0, 0)
  ctx.imageSmoothingEnabled = true
  const pw = W - pad.l - pad.r, ph = H - pad.t - pad.b
  ctx.drawImage(img, pad.l, pad.t, pw, ph)
  const sx = s => pad.l + (s - o.s0) / (o.s1 - o.s0) * pw
  ctx.font = '11px JetBrains Mono, monospace'
  ctx.fillStyle = COR.apagado
  // Marcas só nas bordas, para não cobrir os zeros.
  for (const s of [0, 0.5, 1]) {
    ctx.strokeStyle = s === 0.5 ? COR.vermelho : COR.apagado
    ctx.lineWidth = s === 0.5 ? 2 : 1
    ctx.beginPath(); ctx.moveTo(sx(s), H - pad.b); ctx.lineTo(sx(s), H - pad.b + 8); ctx.stroke()
    ctx.beginPath(); ctx.moveTo(sx(s), pad.t - 8); ctx.lineTo(sx(s), pad.t); ctx.stroke()
    ctx.fillStyle = s === 0.5 ? COR.vermelho : COR.apagado
    ctx.fillText(s === 0.5 ? '½' : String(s), sx(s) - 4, H - 6)
  }
  ctx.lineWidth = 1
  ctx.setLineDash([])
  for (const t of ticks(0, o.tMax, 5)) {
    const yy = pad.t + ph - (t - 0.5) / (o.tMax - 0.5) * ph
    if (yy >= pad.t && yy <= pad.t + ph) ctx.fillText(String(t), 6, yy + 4)
  }
}
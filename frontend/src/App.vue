<script setup>
import { ref, computed, onMounted, onBeforeUnmount, nextTick, watch } from 'vue'
import pt from './i18n/pt.json'
import en from './i18n/en.json'
import { initEngine, zCurve, findZeros, heatmap, primesCurve, psiCurve } from './engine.js'
import { lineChart, heatChart, COR } from './chart.js'

// ---------- idioma ----------
const saved = (() => { try { return localStorage.getItem('lang') } catch { return null } })()
const lang = ref(saved || (navigator.language?.startsWith('pt') ? 'pt' : 'en'))
const t = computed(() => (lang.value === 'en' ? en : pt))
const locale = computed(() => t.value.meta.lang)
function trocarIdioma() {
  lang.value = lang.value === 'pt' ? 'en' : 'pt'
  try { localStorage.setItem('lang', lang.value) } catch {}
}
watch(lang, () => {
  document.documentElement.lang = t.value.meta.lang
  document.title = t.value.meta.title
  nextTick(redesenharTudo)
}, { immediate: true })
const num = (v, d = 0) => Number(v).toLocaleString(locale.value, { minimumFractionDigits: d, maximumFractionDigits: d })

// ---------- navegação ----------
const ids = ['inicio', 'riemann', 'historia', 'claude', 'quase', 'milenio', 'matematicos', 'lab', 'paisagem', 'primos', 'formula']
const romanos = ['0', 'I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X']
const ativo = ref('inicio')
const ir = id => document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' })
const iniciais = n => n.split(/\s+(?:e|and)\s+|\s+/).filter(w => /^[A-ZÀ-Ú]/.test(w)).map(w => w[0]).slice(0, 2).join('')

// ---------- motor ----------
const motor = ref('none')
const erro = ref('')

// ---------- VII laboratório ----------
const ate = ref(60)
const labZeros = ref([])
const labEsperado = ref(0)
const labCurva = ref([])
const labCarregando = ref(false)
const labHover = ref(null)
const cvLab = ref(null)
let labMap = null
const ODLYZKO = [14.134725141734693, 21.022039638771555, 25.010857580145688, 30.424876125859513, 32.935061587739189,
  37.586178158825671, 40.918719012147495, 43.327073280914999, 48.005150881167159, 49.773832477672302]
const tabela = computed(() => labZeros.value.slice(0, 10).map((z, i) => ({ n: i + 1, calc: z, real: ODLYZKO[i], erro: Math.abs(z - ODLYZKO[i]) })))

async function calcularLab() {
  labCarregando.value = true
  try {
    const [c, z] = await Promise.all([zCurve(10, ate.value, 0.02), findZeros(ate.value)])
    const pts = []
    for (let i = 0; i < c.length; i += 2) pts.push([c[i], c[i + 1]])
    labCurva.value = pts
    labZeros.value = z.zeros
    labEsperado.value = z.expected
  } catch (e) { erro.value = String(e) }
  labCarregando.value = false
  desenharLab()
}
function desenharLab() {
  if (!cvLab.value || !labCurva.value.length) return
  labMap = lineChart(cvLab.value, [{ data: labCurva.value, color: COR.tinta, width: 1.4 }],
    { xmin: 10, xmax: ate.value, symmetric: true, dots: labZeros.value, cursor: labHover.value?.t, locale: locale.value })
}
function moverLab(e) {
  if (!labMap || !labCurva.value.length) return
  const tt = labMap.toX(e.clientX - cvLab.value.getBoundingClientRect().left)
  const p = labCurva.value.reduce((a, b) => (Math.abs(b[0] - tt) < Math.abs(a[0] - tt) ? b : a))
  labHover.value = { t: p[0], z: p[1] }
  desenharLab()
}

// ---------- VIII paisagem ----------
const cvHeat = ref(null)
const heatT = ref(40)
let heatVals = null
const HW = 120, HH = 360
async function calcularHeat() {
  try { heatVals = await heatmap(heatT.value, HW, HH) } catch (e) { erro.value = String(e) }
  desenharHeat()
}
function desenharHeat() {
  if (cvHeat.value && heatVals) heatChart(cvHeat.value, heatVals, HW, HH, { s0: -0.5, s1: 1.5, tMax: heatT.value })
}

// ---------- IX primos ----------
const cvPrimos = ref(null)
const expPrimos = ref(4)
const primosDados = ref([])
const primosTabela = ref([])
async function calcularPrimos() {
  try {
    const max = 10 ** expPrimos.value
    const c = await primesCurve(max, 400)
    const rows = []
    for (let i = 0; i < c.length; i += 4) rows.push([c[i], c[i + 1], c[i + 2], c[i + 3]])
    primosDados.value = rows
    const tab = []
    for (let k = 2; k <= expPrimos.value; k++) {
      const r = await primesCurve(10 ** k, 2)
      tab.push({ x: 10 ** k, pi: r[5], li: r[6] - r[5], r: r[7] - r[5] })
    }
    primosTabela.value = tab
  } catch (e) { erro.value = String(e) }
  desenharPrimos()
}
function desenharPrimos() {
  if (!cvPrimos.value || !primosDados.value.length) return
  const d = primosDados.value
  const lim = x => Math.sqrt(x) * Math.log(x) / (8 * Math.PI)
  lineChart(cvPrimos.value, [
    { data: d.map(r => [r[0], lim(r[0])]), color: COR.tinta, width: 1, dash: [1, 3] },
    { data: d.map(r => [r[0], -lim(r[0])]), color: COR.tinta, width: 1, dash: [1, 3] },
    { data: d.map(r => [r[0], r[2] - r[1]]), color: COR.apagado, width: 1.3, dash: [5, 4] },
    { data: d.map(r => [r[0], r[3] - r[1]]), color: COR.vermelho, width: 1.4 }
  ], { xmin: 2, locale: locale.value })
}

// ---------- X fórmula explícita ----------
const cvPsi = ref(null)
const todosZeros = ref([])
const kZeros = ref(10)
const xPsi = ref(50)
const tocando = ref(false)
let timer = null
async function prepararFormula() {
  try { todosZeros.value = (await findZeros(600)).zeros } catch (e) { erro.value = String(e) }
  desenharPsi()
}
async function desenharPsi() {
  if (!cvPsi.value || !todosZeros.value.length) return
  const c = await psiCurve(xPsi.value, 900, todosZeros.value, kZeros.value)
  const real = [], aprox = []
  for (let i = 0; i < c.length; i += 3) { real.push([c[i], c[i + 1]]); aprox.push([c[i], c[i + 2]]) }
  lineChart(cvPsi.value, [
    { data: real, color: COR.tinta, width: 1.6 },
    { data: aprox, color: COR.vermelho, width: 1.3 }
  ], { xmin: 1.5, xmax: xPsi.value, locale: locale.value })
}
function animar() {
  if (tocando.value) { clearInterval(timer); tocando.value = false; return }
  if (kZeros.value >= todosZeros.value.length) kZeros.value = 0
  tocando.value = true
  timer = setInterval(() => {
    kZeros.value = Math.min(todosZeros.value.length, kZeros.value + (kZeros.value < 20 ? 1 : 4))
    desenharPsi()
    if (kZeros.value >= todosZeros.value.length) { clearInterval(timer); tocando.value = false }
  }, 90)
}

function redesenharTudo() { desenharLab(); desenharHeat(); desenharPrimos(); desenharPsi() }

onMounted(async () => {
  const obs = new IntersectionObserver(es => es.forEach(e => { if (e.isIntersecting) ativo.value = e.target.id }), { rootMargin: '-40% 0px -55% 0px' })
  ids.forEach(id => { const el = document.getElementById(id); if (el) obs.observe(el) })
  motor.value = await initEngine()
  if (motor.value === 'none') return
  await calcularLab()
  await calcularHeat()
  await calcularPrimos()
  await prepararFormula()
  window.addEventListener('resize', redesenharTudo)
})
onBeforeUnmount(() => { clearInterval(timer); window.removeEventListener('resize', redesenharTudo) })
</script>

<template>
  <div class="pagina">
    <aside class="indice">
      <div class="marca">ζ(s)</div>
      <nav>
        <a v-for="(id, i) in ids" :key="id" :class="{ on: ativo === id }" @click="ir(id)">
          <span class="num">{{ romanos[i] }}</span>{{ t.nav[id] }}
        </a>
      </nav>
      <button class="idioma" @click="trocarIdioma">{{ t.meta.toggle }}</button>
      <p class="rodape-indice">Rust · WebAssembly · Vue 3<br />{{ t.meta.author }}</p>
    </aside>
    <button class="idioma movel" @click="trocarIdioma">{{ t.meta.toggle }}</button>

    <main>
      <section id="inicio" class="capa">
        <p class="sobrelinha">{{ t.capa.kicker }}</p>
        <h1>{{ t.capa.title1 }}<br />{{ t.capa.title2 }} <em>½</em>?</h1>
        <p class="lead">{{ t.capa.lead }}</p>
        <div class="formula-capa">ζ(s) = <span class="frac"><span>1</span><span>1<sup>s</sup></span></span> + <span class="frac"><span>1</span><span>2<sup>s</sup></span></span> + <span class="frac"><span>1</span><span>3<sup>s</sup></span></span> + ⋯</div>
        <div class="selos"><div v-for="s in t.capa.stats" :key="s[0]"><b>{{ s[0] }}</b><span>{{ s[1] }}</span></div></div>
      </section>

      <section id="riemann">
        <header class="cab"><span class="romano">I</span><h2>{{ t.riemann.title }}</h2></header>
        <div class="duas">
          <div class="texto"><p v-for="(p, i) in t.riemann.paragraphs" :key="p" :class="{ capitular: i === 0 }">{{ p }}</p></div>
          <figure class="faixa">
            <svg viewBox="0 0 260 360" role="img" :aria-label="t.riemann.stripCaption">
              <rect x="60" y="10" width="140" height="320" fill="#ebe2cf" />
              <line x1="20" y1="330" x2="245" y2="330" stroke="#1b1a17" />
              <line x1="60" y1="10" x2="60" y2="335" stroke="#1b1a17" />
              <line x1="200" y1="10" x2="200" y2="335" stroke="#1b1a17" stroke-dasharray="3 3" />
              <line x1="130" y1="10" x2="130" y2="335" stroke="#b3261e" stroke-width="1.5" />
              <circle v-for="z in ODLYZKO" :key="z" cx="130" :cy="330 - z * 6" r="4" fill="#b3261e" />
              <text x="56" y="350" class="eixo">0</text><text x="122" y="350" class="eixo">½</text>
              <text x="196" y="350" class="eixo">1</text><text x="236" y="322" class="eixo">Re</text>
              <text x="40" y="22" class="eixo">Im</text><text x="138" y="26" class="nota">{{ t.riemann.stripLabel }}</text>
            </svg>
            <figcaption>{{ t.riemann.stripCaption }}</figcaption>
          </figure>
        </div>
        <blockquote class="euler">
          <div class="eq">∑ <span class="frac"><span>1</span><span>n<sup>s</sup></span></span> = ∏<sub>{{ t.riemann.eulerSub }}</sub> <span class="frac"><span>1</span><span>1 − p<sup>−s</sup></span></span></div>
          <p>{{ t.riemann.euler }}</p>
        </blockquote>
      </section>

      <section id="historia">
        <header class="cab"><span class="romano">II</span><h2>{{ t.historia.title }}</h2></header>
        <p class="intro">{{ t.historia.intro }}</p>
        <ol class="linha">
          <li v-for="m in t.historia.milestones" :key="m.year" :class="{ ia: m.ai }">
            <div class="ano">{{ m.year }}</div>
            <div class="corpo">
              <div class="quem">{{ m.who }} <span>{{ m.text }}</span></div>
              <div v-if="m.pct !== null" class="regua">
                <div class="preenche" :style="{ width: Math.max(m.pct, 0.6) + '%' }"></div>
                <span class="valor">{{ m.pct < 2 ? '> 0%' : num(m.pct, 1) + '%' }}</span>
              </div>
              <div v-else class="sem">{{ t.historia.infinite }}</div>
            </div>
          </li>
        </ol>
      </section>

      <section id="claude" class="destaque">
        <header class="cab"><span class="romano">III</span><h2>{{ t.claude.title }}</h2></header>
        <div class="numero-grande">{{ num(41.6, 1) }} <span>→</span> <em>{{ num(67.2, 1) }}</em><small>%</small></div>
        <div class="colunas"><p v-for="p in t.claude.paragraphs" :key="p">{{ p }}</p></div>
        <ul class="fatos"><li v-for="f in t.claude.facts" :key="f[1]"><b>{{ f[0] }}</b> {{ f[1] }}</li></ul>
      </section>

      <section id="quase">
        <header class="cab"><span class="romano">IV</span><h2>{{ t.quase.title }}</h2></header>
        <div class="duas">
          <div class="texto"><p v-for="p in t.quase.paragraphs" :key="p">{{ p }}</p></div>
          <figure class="faixa">
            <svg viewBox="0 0 260 300" role="img" :aria-label="t.quase.caption">
              <rect x="60" y="10" width="140" height="260" fill="#ebe2cf" />
              <rect x="160" y="10" width="40" height="260" fill="#f6f1e6" stroke="#b3261e" stroke-dasharray="4 3" />
              <line x1="60" y1="270" x2="230" y2="270" stroke="#1b1a17" />
              <line x1="130" y1="10" x2="130" y2="270" stroke="#b3261e" stroke-width="1.5" />
              <text x="164" y="140" class="nota">{{ t.quase.noZeros[0] }}</text><text x="164" y="154" class="nota">{{ t.quase.noZeros[1] }}</text>
              <text x="122" y="288" class="eixo">½</text><text x="150" y="288" class="eixo">1−δ</text><text x="196" y="288" class="eixo">1</text>
            </svg>
            <figcaption>{{ t.quase.caption }}</figcaption>
          </figure>
        </div>
      </section>

      <section id="milenio">
        <header class="cab"><span class="romano">V</span><h2>{{ t.milenio.title }}</h2></header>
        <p class="intro">{{ t.milenio.intro }}</p>
        <div class="problemas">
          <article v-for="(m, i) in t.milenio.items" :key="m.name" :class="{ feito: m.solved, anunciado: m.claimed }">
            <div class="idx">{{ String(i + 1).padStart(2, '0') }}</div>
            <div>
              <h3>{{ m.name }}</h3>
              <p>{{ m.summary }}</p>
              <div class="pe"><span class="carimbo">{{ m.solved ? t.milenio.solved : m.claimed ? t.milenio.claimed : t.milenio.open }}</span><span class="nomes">{{ m.people.join(', ') }}</span></div>
            </div>
          </article>
        </div>
      </section>

      <section id="matematicos">
        <header class="cab"><span class="romano">VI</span><h2>{{ t.matematicos.title }}</h2></header>
        <div class="fichas">
          <article v-for="m in t.matematicos.items" :key="m.name">
            <div class="mono">{{ iniciais(m.name) }}</div>
            <h3>{{ m.name }}</h3><div class="anos">{{ m.years }}</div><p>{{ m.text }}</p>
          </article>
        </div>
      </section>

      <p v-if="motor === 'none'" class="aviso">{{ t.lab.engine.none }}</p>

      <section id="lab">
        <header class="cab"><span class="romano">VII</span><h2>{{ t.lab.title }}</h2></header>
        <p class="intro">{{ t.lab.intro }}</p>
        <p class="motor" :class="motor">● {{ t.lab.engine[motor] }}</p>
        <div class="controle">
          <label>{{ t.lab.range }} <b>{{ ate }}</b></label>
          <input type="range" min="20" max="400" step="1" v-model.number="ate" @change="calcularLab" />
          <span class="status">{{ labCarregando ? t.lab.computing : labZeros.length + ' ' + t.lab.zerosWord }}</span>
        </div>
        <div class="grafico">
          <canvas ref="cvLab" class="alto" @mousemove="moverLab" @mouseleave="labHover = null; desenharLab()"></canvas>
          <div v-if="labHover" class="leitura">t = {{ num(labHover.t, 2) }} · Z(t) = {{ num(labHover.z, 4) }}</div>
        </div>
        <div class="duas-tab">
          <table class="zeros">
            <thead><tr><th v-for="c in t.lab.cols" :key="c">{{ c }}</th></tr></thead>
            <tbody>
              <tr v-for="r in tabela" :key="r.n">
                <td>{{ r.n }}</td><td>{{ num(r.calc, 9) }}</td><td>{{ num(r.real, 9) }}</td><td>{{ r.erro.toExponential(1) }}</td>
              </tr>
            </tbody>
          </table>
          <div class="conferencia">
            <h3>{{ t.lab.check }}</h3>
            <p>{{ t.lab.checkText }}</p>
            <div class="placar">
              <div><b>{{ labZeros.length }}</b><span>{{ t.lab.found }}</span></div>
              <div><b>{{ num(labEsperado, 2) }}</b><span>{{ t.lab.expected }} (T = {{ ate }})</span></div>
            </div>
          </div>
        </div>
        <p class="nota-rodape">{{ t.lab.note }}</p>
      </section>

      <section id="paisagem">
        <header class="cab"><span class="romano">VIII</span><h2>{{ t.paisagem.title }}</h2></header>
        <div class="duas">
          <div class="texto">
            <p>{{ t.paisagem.intro }}</p>
            <div class="controle">
              <label>t ≤ <b>{{ heatT }}</b></label>
              <input type="range" min="20" max="100" step="5" v-model.number="heatT" @change="calcularHeat" />
            </div>
            <p class="eixos">↔ {{ t.paisagem.axisS }} &nbsp; ↕ {{ t.paisagem.axisT }}</p>
          </div>
          <div class="grafico sem-borda"><canvas ref="cvHeat" class="mapa"></canvas></div>
        </div>
      </section>

      <section id="primos">
        <header class="cab"><span class="romano">IX</span><h2>{{ t.primos.title }}</h2></header>
        <p class="intro">{{ t.primos.intro }}</p>
        <div class="controle">
          <label>{{ t.primos.range }} <b>10<sup>{{ expPrimos }}</sup></b></label>
          <input type="range" min="2" max="7" step="1" v-model.number="expPrimos" @change="calcularPrimos" />
        </div>
        <ul class="legenda">
          <li><i class="l-tracejada"></i>{{ t.primos.legend.li }}</li>
          <li><i class="l-vermelha"></i>{{ t.primos.legend.r }}</li>
          <li><i class="l-pontilhada"></i>{{ t.primos.legend.bound }}</li>
        </ul>
        <div class="grafico"><canvas ref="cvPrimos" class="alto"></canvas></div>
        <table class="zeros">
          <thead><tr><th v-for="c in t.primos.tableCols" :key="c">{{ c }}</th></tr></thead>
          <tbody>
            <tr v-for="r in primosTabela" :key="r.x">
              <td>10<sup>{{ Math.round(Math.log10(r.x)) }}</sup></td><td>{{ num(r.pi) }}</td><td>{{ num(r.li, 1) }}</td><td>{{ num(r.r, 1) }}</td>
            </tr>
          </tbody>
        </table>
      </section>

      <section id="formula">
        <header class="cab"><span class="romano">X</span><h2>{{ t.formula.title }}</h2></header>
        <p class="intro">{{ t.formula.intro }}</p>
        <p class="formula-linha">ψ(x) = x − Σ<sub>ρ</sub> x<sup>ρ</sup>/ρ − ln 2π − ½ ln(1 − x<sup>−2</sup>)</p>
        <div class="controle">
          <label>{{ t.formula.zerosUsed }} <b>{{ kZeros }}</b></label>
          <input type="range" min="0" :max="todosZeros.length || 300" step="1" v-model.number="kZeros" @input="desenharPsi" />
          <button class="botao" @click="animar">{{ tocando ? t.formula.pause : t.formula.play }}</button>
        </div>
        <div class="controle">
          <label>{{ t.formula.xMax }} <b>{{ xPsi }}</b></label>
          <input type="range" min="20" max="120" step="10" v-model.number="xPsi" @change="desenharPsi" />
        </div>
        <ul class="legenda">
          <li><i class="l-tinta"></i>{{ t.formula.legend.real }}</li>
          <li><i class="l-vermelha"></i>{{ t.formula.legend.approx }}</li>
        </ul>
        <div class="grafico"><canvas ref="cvPsi" class="alto"></canvas></div>
      </section>

      <p v-if="erro" class="aviso">{{ erro }}</p>
      <footer><p>{{ t.footer }}</p></footer>
    </main>
  </div>
</template>

<style>
:root {
  --papel: #f3ede1; --papel-2: #ebe2cf; --tinta: #1b1a17; --tinta-2: #4a453c;
  --apagado: #8a8070; --linha: #d8cfbd; --vermelho: #b3261e;
  --serif: 'Fraunces', Georgia, serif; --texto: 'Newsreader', Georgia, serif; --mono: 'JetBrains Mono', monospace;
}
* { box-sizing: border-box; }
html { scroll-behavior: smooth; }
body { margin: 0; background: var(--papel); color: var(--tinta); font-family: var(--texto); font-size: 19px; line-height: 1.65; }
::selection { background: var(--vermelho); color: var(--papel); }

.pagina { display: grid; grid-template-columns: 240px minmax(0, 1fr); min-height: 100vh; }
.indice { position: sticky; top: 0; height: 100vh; padding: 40px 28px; border-right: 1px solid var(--linha); display: flex; flex-direction: column; overflow-y: auto; }
.marca { font-family: var(--serif); font-size: 34px; font-style: italic; margin-bottom: 32px; }
.indice nav { display: flex; flex-direction: column; gap: 2px; }
.indice a { cursor: pointer; font-family: var(--serif); font-size: 15px; color: var(--apagado); padding: 5px 0; display: flex; gap: 12px; transition: color .2s; }
.indice a:hover { color: var(--tinta); }
.indice a.on { color: var(--vermelho); }
.indice .num { font-family: var(--mono); font-size: 11px; width: 28px; padding-top: 3px; }
.idioma { margin-top: 24px; align-self: flex-start; background: none; border: 1px solid var(--tinta); color: var(--tinta); font-family: var(--mono); font-size: 12px; padding: 6px 12px; cursor: pointer; }
.idioma:hover { background: var(--tinta); color: var(--papel); }
.idioma.movel { display: none; }
.rodape-indice { margin-top: auto; padding-top: 24px; font-family: var(--mono); font-size: 11px; color: var(--apagado); line-height: 1.6; }

main { padding: 0 72px 80px; max-width: 1080px; }
section { padding: 96px 0 40px; border-bottom: 1px solid var(--linha); }
.cab { display: flex; align-items: baseline; gap: 20px; margin-bottom: 32px; }
.romano { font-family: var(--serif); font-style: italic; font-size: 20px; color: var(--vermelho); min-width: 36px; }
h1, h2, h3 { font-family: var(--serif); font-weight: 400; margin: 0; }
h2 { font-size: 42px; line-height: 1.1; letter-spacing: -0.01em; }
h3 { font-size: 21px; font-weight: 500; }
.intro { font-size: 21px; color: var(--tinta-2); max-width: 720px; margin: 0 0 32px; }

.capa { padding-top: 120px; min-height: 92vh; }
.sobrelinha { font-family: var(--mono); font-size: 12px; letter-spacing: .08em; text-transform: uppercase; color: var(--vermelho); }
.capa h1 { font-size: clamp(56px, 8vw, 104px); line-height: .98; letter-spacing: -0.03em; margin: 20px 0 32px; }
.capa h1 em { color: var(--vermelho); }
.lead { font-size: 23px; max-width: 620px; color: var(--tinta-2); }
.formula-capa { font-family: var(--serif); font-size: 30px; margin: 48px 0; display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.frac { display: inline-flex; flex-direction: column; text-align: center; font-size: .8em; vertical-align: middle; }
.frac span:first-child { border-bottom: 1px solid currentColor; padding: 0 4px; }
.selos { display: grid; grid-template-columns: repeat(3, 1fr); border-top: 1px solid var(--tinta); margin-top: 56px; }
.selos div { padding: 20px 20px 0 0; }
.selos b { display: block; font-family: var(--serif); font-weight: 400; font-size: 40px; }
.selos span { font-family: var(--mono); font-size: 12px; color: var(--apagado); }

.duas { display: grid; grid-template-columns: minmax(0, 1.4fr) minmax(0, 1fr); gap: 56px; align-items: start; }
.texto p { margin: 0 0 18px; }
.capitular::first-letter { font-family: var(--serif); float: left; font-size: 74px; line-height: .8; padding: 8px 10px 0 0; color: var(--vermelho); }
.faixa { margin: 0; position: sticky; top: 40px; }
.faixa svg { width: 100%; }
.faixa .eixo { font-family: var(--mono); font-size: 11px; fill: var(--apagado); }
.faixa .nota { font-family: var(--serif); font-style: italic; font-size: 12px; fill: var(--vermelho); }
figcaption { font-size: 15px; color: var(--apagado); font-style: italic; margin-top: 12px; }
.euler { margin: 48px 0 0; padding: 32px 0 0; border-top: 1px solid var(--linha); }
.euler .eq { font-family: var(--serif); font-size: 30px; display: flex; align-items: center; gap: 12px; flex-wrap: wrap; }
.euler p { color: var(--tinta-2); max-width: 640px; }

.linha { list-style: none; padding: 0; margin: 0; }
.linha li { display: grid; grid-template-columns: 120px 1fr; gap: 24px; padding: 20px 0; border-top: 1px solid var(--linha); }
.linha .ano { font-family: var(--mono); font-size: 14px; color: var(--apagado); padding-top: 4px; }
.linha .quem { font-family: var(--serif); font-size: 21px; margin-bottom: 10px; }
.linha .quem span { font-family: var(--texto); font-size: 17px; color: var(--apagado); margin-left: 8px; }
.regua { position: relative; height: 10px; background: var(--papel-2); }
.preenche { height: 100%; background: var(--tinta); transition: width 1s ease; }
.valor { position: absolute; right: 0; top: -26px; font-family: var(--mono); font-size: 13px; }
.sem { font-family: var(--mono); font-size: 12px; color: var(--apagado); }
.linha li.ia .preenche { background: var(--vermelho); }
.linha li.ia .quem, .linha li.ia .valor { color: var(--vermelho); }

.destaque { background: var(--tinta); color: var(--papel); margin: 0 -72px; padding: 96px 72px 72px; border: none; }
.destaque .cab h2 { color: var(--papel); }
.numero-grande { font-family: var(--serif); font-size: clamp(64px, 10vw, 132px); line-height: 1; margin: 16px 0 48px; letter-spacing: -0.03em; }
.numero-grande span { color: var(--apagado); font-size: .5em; vertical-align: middle; }
.numero-grande em { color: #e5574d; }
.numero-grande small { font-size: .4em; color: var(--apagado); }
.colunas { columns: 2; column-gap: 48px; }
.colunas p { margin: 0 0 18px; break-inside: avoid; color: #d9d1c1; }
.fatos { list-style: none; padding: 0; margin: 40px 0 0; display: grid; grid-template-columns: repeat(4, 1fr); border-top: 1px solid #3a372f; }
.fatos li { padding: 18px 0 0; font-family: var(--mono); font-size: 12px; color: var(--apagado); }
.fatos b { display: block; font-family: var(--serif); font-weight: 400; font-size: 30px; color: var(--papel); }

.problemas article { display: grid; grid-template-columns: 72px 1fr; gap: 16px; padding: 28px 0; border-top: 1px solid var(--linha); }
.problemas .idx { font-family: var(--serif); font-size: 40px; color: var(--linha); line-height: 1; }
.problemas p { margin: 8px 0 14px; color: var(--tinta-2); max-width: 720px; }
.pe { display: flex; align-items: center; gap: 16px; flex-wrap: wrap; }
.carimbo { font-family: var(--mono); font-size: 11px; text-transform: uppercase; letter-spacing: .08em; border: 1px solid var(--apagado); color: var(--apagado); padding: 3px 10px; }
.nomes { font-style: italic; color: var(--apagado); font-size: 16px; }
.problemas .feito .idx { color: var(--vermelho); }
.problemas .feito .carimbo { border-color: var(--vermelho); color: var(--vermelho); transform: rotate(-2deg); }

.fichas { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); border-top: 1px solid var(--linha); border-left: 1px solid var(--linha); }
.fichas article { padding: 24px; border-right: 1px solid var(--linha); border-bottom: 1px solid var(--linha); transition: background .2s; }
.fichas article:hover { background: var(--papel-2); }
.mono { width: 48px; height: 48px; border-radius: 50%; border: 1px solid var(--tinta); display: flex; align-items: center; justify-content: center; font-family: var(--serif); font-style: italic; font-size: 18px; margin-bottom: 16px; }
.fichas .anos { font-family: var(--mono); font-size: 12px; color: var(--vermelho); margin: 4px 0 10px; }
.fichas p { font-size: 16px; color: var(--tinta-2); margin: 0; }

.motor { font-family: var(--mono); font-size: 12px; color: var(--apagado); margin: -16px 0 24px; }
.motor.wasm { color: #2e6b3a; }
.controle { display: flex; align-items: center; gap: 20px; margin-bottom: 16px; font-family: var(--mono); font-size: 13px; }
.controle label { min-width: 210px; }
.controle input { flex: 1; accent-color: var(--vermelho); }
.status { color: var(--vermelho); min-width: 90px; text-align: right; }
.botao { background: var(--vermelho); color: var(--papel); border: none; font-family: var(--mono); font-size: 12px; padding: 7px 16px; cursor: pointer; }
.botao:hover { background: var(--tinta); }
.grafico { position: relative; border-top: 1px solid var(--tinta); border-bottom: 1px solid var(--tinta); margin-top: 8px; }
.grafico.sem-borda { border: none; margin: 0; }
.grafico canvas { width: 100%; display: block; }
canvas.alto { height: 340px; cursor: crosshair; }
canvas.mapa { height: 560px; }
.leitura { position: absolute; top: 10px; right: 10px; font-family: var(--mono); font-size: 12px; background: var(--papel); padding: 4px 8px; border: 1px solid var(--linha); }
.legenda { list-style: none; padding: 0; margin: 8px 0; display: flex; gap: 24px; flex-wrap: wrap; font-family: var(--mono); font-size: 12px; color: var(--tinta-2); }
.legenda i { display: inline-block; width: 22px; height: 0; vertical-align: middle; margin-right: 8px; border-top: 2px solid var(--tinta); }
.legenda .l-vermelha { border-color: var(--vermelho); }
.legenda .l-tracejada { border-top: 2px dashed var(--apagado); }
.legenda .l-pontilhada { border-top: 2px dotted var(--tinta); }
.formula-linha { font-family: var(--serif); font-style: italic; font-size: 26px; margin: 0 0 28px; }
.eixos { font-family: var(--mono); font-size: 12px; color: var(--apagado); }

.duas-tab { display: grid; grid-template-columns: minmax(0, 1.3fr) minmax(0, 1fr); gap: 40px; margin-top: 32px; }
.zeros { width: 100%; border-collapse: collapse; margin-top: 24px; font-family: var(--mono); font-size: 13px; }
.duas-tab .zeros { margin-top: 0; }
.zeros th { text-align: left; font-weight: 400; color: var(--apagado); font-size: 11px; text-transform: uppercase; letter-spacing: .06em; padding: 8px 8px 8px 0; border-bottom: 1px solid var(--tinta); }
.zeros td { padding: 7px 8px 7px 0; border-bottom: 1px solid var(--linha); }
.zeros td:last-child { color: var(--vermelho); }
.conferencia { border: 1px solid var(--tinta); padding: 24px; align-self: start; }
.conferencia p { font-size: 16px; color: var(--tinta-2); }
.placar { display: grid; grid-template-columns: 1fr 1fr; border-top: 1px solid var(--linha); padding-top: 16px; }
.placar b { display: block; font-family: var(--serif); font-weight: 400; font-size: 36px; }
.placar span { font-family: var(--mono); font-size: 11px; color: var(--apagado); }
.nota-rodape { font-size: 15px; font-style: italic; color: var(--apagado); }
.aviso { font-family: var(--mono); font-size: 13px; color: var(--vermelho); border: 1px solid var(--vermelho); padding: 12px 16px; margin-top: 32px; }

.problemas .anunciado .idx { color: var(--tinta-2); }
.problemas .anunciado .carimbo { border-style: dashed; border-color: var(--vermelho); color: var(--vermelho); }

footer { padding: 56px 0; font-style: italic; color: var(--apagado); }

@media (max-width: 900px) {
  .pagina { grid-template-columns: 1fr; }
  .indice { display: none; }
  .idioma.movel { display: block; position: absolute; top: 16px; right: 16px; margin: 0; z-index: 2; background: var(--papel); }
  main { padding: 0 24px 60px; }
  .destaque { margin: 0 -24px; padding: 72px 24px; }
  .duas, .duas-tab { grid-template-columns: 1fr; }
  .colunas { columns: 1; }
  .faixa { position: static; }
  .selos, .fatos { grid-template-columns: 1fr 1fr; }
  .controle { flex-wrap: wrap; }
  .controle label { min-width: 0; }
  h2 { font-size: 32px; }
}
</style>
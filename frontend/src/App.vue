<script setup>
import { ref, computed, onMounted, nextTick } from 'vue'

const c = ref(null)
const ativo = ref('inicio')
const ate = ref(60)
const zeros = ref([])
const pontos = ref([])
const canvas = ref(null)
const hover = ref(null)
const carregando = ref(false)

const secoes = [
  ['inicio', '0', 'Início'],
  ['riemann', 'I', 'A hipótese'],
  ['historia', 'II', 'Um século de avanços'],
  ['claude', 'III', 'Agosto de 2026'],
  ['quase', 'IV', 'Quase-Riemann'],
  ['milenio', 'V', 'Os sete problemas'],
  ['matematicos', 'VI', 'Quem construiu isso'],
  ['lab', 'VII', 'Laboratório']
]

const marcos = [
  { ano: '1914', quem: 'Hardy', txt: 'Infinitos zeros na reta crítica', pct: null },
  { ano: '1942', quem: 'Selberg', txt: 'Uma proporção positiva, ainda que minúscula', pct: 1 },
  { ano: '1974', quem: 'Levinson', txt: 'Pelo menos um terço', pct: 33.3 },
  { ano: '1989', quem: 'Conrey', txt: 'Mais de 40%', pct: 40 },
  { ano: 'até 2026', quem: 'vários', txt: 'Melhor limite incondicional reportado', pct: 41.6 },
  { ano: '2026', quem: 'Claude', txt: 'Versão de pesquisa da Anthropic', pct: 67.2, ia: true }
]

const conhecidos = [14.134725, 21.02204, 25.010858, 30.424876, 32.935062, 37.586178, 40.918719, 43.327073, 48.005151, 49.773832]

const tabela = computed(() => zeros.value.slice(0, 10).map((z, i) => ({
  n: i + 1, calc: z, real: conhecidos[i], erro: conhecidos[i] ? Math.abs(z - conhecidos[i]) : null
})))

const iniciais = n => n.split(/\s+e\s+|\s+/).filter(w => /^[A-ZÀ-Ú]/.test(w)).map(w => w[0]).slice(0, 2).join('')
const resolvido = m => m.status.startsWith('RESOLVIDA')

function ir(id) { document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' }) }

async function calcular() {
  carregando.value = true
  const [zr, zz] = await Promise.all([
    fetch(`/api/z?from=10&to=${ate.value}&step=0.02`).then(r => r.json()),
    fetch(`/api/zeros?to=${ate.value}`).then(r => r.json())
  ])
  pontos.value = zr.points
  zeros.value = zz.zeros
  carregando.value = false
  await nextTick()
  desenhar()
}

function desenhar() {
  const cv = canvas.value
  if (!cv) return
  const dpr = window.devicePixelRatio || 1
  const W = cv.clientWidth, H = cv.clientHeight
  cv.width = W * dpr; cv.height = H * dpr
  const ctx = cv.getContext('2d')
  ctx.scale(dpr, dpr)
  ctx.clearRect(0, 0, W, H)
  const pts = pontos.value
  if (!pts.length) return
  const pad = 36
  const ymax = Math.max(...pts.map(p => Math.abs(p[1]))) * 1.08
  const x = t => pad + (t - 10) / (ate.value - 10) * (W - pad * 2)
  const y = v => H / 2 - v / ymax * (H / 2 - 20)

  ctx.font = '11px JetBrains Mono'
  ctx.fillStyle = '#8a8070'
  ctx.strokeStyle = '#d8cfbd'
  ctx.lineWidth = 1
  const passo = ate.value > 120 ? 20 : 10
  for (let t = Math.ceil(10 / passo) * passo; t <= ate.value; t += passo) {
    ctx.beginPath(); ctx.moveTo(x(t), 16); ctx.lineTo(x(t), H - 22); ctx.setLineDash([2, 4]); ctx.stroke()
    ctx.setLineDash([]); ctx.fillText(String(t), x(t) - 8, H - 6)
  }
  ctx.strokeStyle = '#1b1a17'
  ctx.beginPath(); ctx.moveTo(pad, H / 2); ctx.lineTo(W - pad, H / 2); ctx.stroke()

  ctx.strokeStyle = '#1b1a17'; ctx.lineWidth = 1.4; ctx.beginPath()
  pts.forEach((p, i) => i ? ctx.lineTo(x(p[0]), y(p[1])) : ctx.moveTo(x(p[0]), y(p[1])))
  ctx.stroke()

  ctx.fillStyle = '#b3261e'
  zeros.value.forEach(z => { ctx.beginPath(); ctx.arc(x(z), H / 2, 4, 0, Math.PI * 2); ctx.fill() })

  if (hover.value) {
    ctx.strokeStyle = '#b3261e'; ctx.lineWidth = 1
    ctx.beginPath(); ctx.moveTo(x(hover.value.t), 16); ctx.lineTo(x(hover.value.t), H - 22); ctx.stroke()
  }
  cv._map = { pad, W }
}

function mover(e) {
  const cv = canvas.value
  if (!cv || !cv._map || !pontos.value.length) return
  const r = cv.getBoundingClientRect()
  const { pad, W } = cv._map
  const t = 10 + (e.clientX - r.left - pad) / (W - pad * 2) * (ate.value - 10)
  const p = pontos.value.reduce((a, b) => Math.abs(b[0] - t) < Math.abs(a[0] - t) ? b : a)
  hover.value = { t: p[0], z: p[1] }
  desenhar()
}

onMounted(async () => {
  c.value = await (await fetch('/api/content')).json()
  await nextTick()
  const obs = new IntersectionObserver(es => es.forEach(e => { if (e.isIntersecting) ativo.value = e.target.id }), { rootMargin: '-40% 0px -55% 0px' })
  secoes.forEach(([id]) => { const el = document.getElementById(id); if (el) obs.observe(el) })
  calcular()
  window.addEventListener('resize', desenhar)
})
</script>

<template>
  <div class="pagina">
    <aside class="indice">
      <div class="marca">ζ(s)</div>
      <nav>
        <a v-for="[id, num, nome] in secoes" :key="id" :class="{ on: ativo === id }" @click="ir(id)">
          <span class="num">{{ num }}</span>{{ nome }}
        </a>
      </nav>
      <p class="rodape-indice">Rust · Vue 3<br />Guilherme Savassa Bernal</p>
    </aside>

    <main v-if="c">
      <section id="inicio" class="capa">
        <p class="sobrelinha">Um caderno sobre números primos, 1859 a 2026</p>
        <h1>Todos os zeros<br />estão na reta <em>½</em>?</h1>
        <p class="lead">Há 167 anos, Bernhard Riemann escreveu uma frase que ninguém conseguiu provar. Em 2026, duas inteligências artificiais chegaram mais perto do que qualquer pessoa. Nenhuma chegou lá.</p>
        <div class="formula-capa">ζ(s) = <span class="frac"><span>1</span><span>1<sup>s</sup></span></span> + <span class="frac"><span>1</span><span>2<sup>s</sup></span></span> + <span class="frac"><span>1</span><span>3<sup>s</sup></span></span> + ⋯</div>
        <div class="selos">
          <div><b>1859</b><span>enunciada</span></div>
          <div><b>67,2%</b><span>dos zeros provados na reta</span></div>
          <div><b>1 de 7</b><span>Problemas do Milênio resolvidos</span></div>
        </div>
      </section>

      <section id="riemann">
        <header class="cab"><span class="romano">I</span><h2>{{ c.riemann.titulo }}</h2></header>
        <div class="duas">
          <div class="texto">
            <p v-for="(p, i) in c.riemann.paragrafos" :key="p" :class="{ capitular: i === 0 }">{{ p }}</p>
          </div>
          <figure class="faixa">
            <svg viewBox="0 0 260 360" role="img" aria-label="Faixa crítica do plano complexo com zeros na reta Re(s) = 1/2">
              <rect x="60" y="10" width="140" height="320" fill="#ebe2cf" />
              <line x1="20" y1="330" x2="245" y2="330" stroke="#1b1a17" />
              <line x1="60" y1="10" x2="60" y2="335" stroke="#1b1a17" />
              <line x1="200" y1="10" x2="200" y2="335" stroke="#1b1a17" stroke-dasharray="3 3" />
              <line x1="130" y1="10" x2="130" y2="335" stroke="#b3261e" stroke-width="1.5" />
              <circle v-for="z in conhecidos" :key="z" cx="130" :cy="330 - z * 6" r="4" fill="#b3261e" />
              <text x="56" y="350" class="eixo">0</text>
              <text x="122" y="350" class="eixo">½</text>
              <text x="196" y="350" class="eixo">1</text>
              <text x="236" y="322" class="eixo">Re</text>
              <text x="40" y="22" class="eixo">Im</text>
              <text x="138" y="26" class="nota">reta crítica</text>
            </svg>
            <figcaption>A faixa crítica, 0 &lt; Re(s) &lt; 1. Os pontos vermelhos são os dez primeiros zeros não triviais, e todos caem exatamente em Re(s) = ½. A hipótese diz que isso vale para os infinitos zeros.</figcaption>
          </figure>
        </div>
        <blockquote class="euler">
          <div class="eq">∑ <span class="frac"><span>1</span><span>n<sup>s</sup></span></span> = ∏<sub>p primo</sub> <span class="frac"><span>1</span><span>1 − p<sup>−s</sup></span></span></div>
          <p>O produto de Euler. Do lado esquerdo, todos os inteiros; do lado direito, só os primos. É por essa igualdade que os zeros da zeta contam onde estão os primos.</p>
        </blockquote>
      </section>

      <section id="historia">
        <header class="cab"><span class="romano">II</span><h2>Um século empurrando uma porcentagem</h2></header>
        <p class="intro">Provar que 100% dos zeros estão na reta é a hipótese. Enquanto isso não acontece, matemáticos provam frações cada vez maiores.</p>
        <ol class="linha">
          <li v-for="m in marcos" :key="m.ano" :class="{ ia: m.ia }">
            <div class="ano">{{ m.ano }}</div>
            <div class="corpo">
              <div class="quem">{{ m.quem }} <span>{{ m.txt }}</span></div>
              <div v-if="m.pct !== null" class="regua">
                <div class="preenche" :style="{ width: Math.max(m.pct, 0.6) + '%' }"></div>
                <span class="valor">{{ m.pct < 2 ? '> 0%' : m.pct.toLocaleString('pt-BR') + '%' }}</span>
              </div>
              <div v-else class="sem">infinitos, mas proporção desconhecida</div>
            </div>
          </li>
        </ol>
      </section>

      <section id="claude" class="destaque">
        <header class="cab"><span class="romano">III</span><h2>{{ c.riemann.ia.titulo }}</h2></header>
        <div class="numero-grande">41,6 <span>→</span> <em>67,2</em><small>%</small></div>
        <div class="colunas">
          <p v-for="p in c.riemann.ia.paragrafos" :key="p">{{ p }}</p>
        </div>
        <ul class="fatos">
          <li><b>~60</b> subagentes</li>
          <li><b>~650</b> ideias testadas</li>
          <li><b>1,5</b> dia</li>
          <li><b>Lean</b> verificação formal</li>
        </ul>
      </section>

      <section id="quase">
        <header class="cab"><span class="romano">IV</span><h2>{{ c.quase.titulo }}</h2></header>
        <div class="duas">
          <div class="texto"><p v-for="p in c.quase.paragrafos" :key="p">{{ p }}</p></div>
          <figure class="faixa">
            <svg viewBox="0 0 260 300" role="img" aria-label="Comparação entre a hipótese de Riemann e a quase-Riemann">
              <rect x="60" y="10" width="140" height="260" fill="#ebe2cf" />
              <rect x="160" y="10" width="40" height="260" fill="#f6f1e6" stroke="#b3261e" stroke-dasharray="4 3" />
              <line x1="60" y1="270" x2="230" y2="270" stroke="#1b1a17" />
              <line x1="130" y1="10" x2="130" y2="270" stroke="#b3261e" stroke-width="1.5" />
              <text x="164" y="140" class="nota">sem</text>
              <text x="164" y="154" class="nota">zeros</text>
              <text x="122" y="288" class="eixo">½</text>
              <text x="150" y="288" class="eixo">1−δ</text>
              <text x="196" y="288" class="eixo">1</text>
            </svg>
            <figcaption>Riemann exige que todos os zeros fiquem na linha vermelha. A quase-Riemann pede menos: que exista uma faixa de largura fixa δ, perto de Re(s) = 1, sem nenhum zero.</figcaption>
          </figure>
        </div>
      </section>

      <section id="milenio">
        <header class="cab"><span class="romano">V</span><h2>Os sete Problemas do Milênio</h2></header>
        <p class="intro">Em 2000, o Clay Mathematics Institute ofereceu US$ 1 milhão para cada um. Vinte e seis anos depois, a conta está assim:</p>
        <div class="problemas">
          <article v-for="(m, i) in c.milenio" :key="m.nome" :class="{ feito: resolvido(m) }">
            <div class="idx">{{ String(i + 1).padStart(2, '0') }}</div>
            <div>
              <h3>{{ m.nome }}</h3>
              <p>{{ m.resumo }}</p>
              <div class="pe">
                <span class="carimbo">{{ resolvido(m) ? 'Resolvido' : 'Em aberto' }}</span>
                <span class="nomes">{{ m.matematicos.join(', ') }}</span>
              </div>
            </div>
          </article>
        </div>
      </section>

      <section id="matematicos">
        <header class="cab"><span class="romano">VI</span><h2>Quem construiu isso</h2></header>
        <div class="fichas">
          <article v-for="m in c.matematicos" :key="m.nome">
            <div class="mono">{{ iniciais(m.nome) }}</div>
            <h3>{{ m.nome }}</h3>
            <div class="anos">{{ m.anos }}</div>
            <p>{{ m.contribuicao }}</p>
          </article>
        </div>
      </section>

      <section id="lab">
        <header class="cab"><span class="romano">VII</span><h2>Laboratório: encontrando zeros em Rust</h2></header>
        <p class="intro">A função Z(t) de Riemann-Siegel é real e tem o mesmo módulo de ζ(½ + it). Cada vez que ela cruza o eixo, há um zero da zeta na reta crítica. O backend calcula a curva e acha os cruzamentos por bissecção.</p>
        <div class="controle">
          <label>Calcular de t = 10 até <b>{{ ate }}</b></label>
          <input type="range" min="20" max="200" step="1" v-model.number="ate" @change="calcular" />
          <span class="status">{{ carregando ? 'calculando…' : zeros.length + ' zeros' }}</span>
        </div>
        <div class="grafico">
          <canvas ref="canvas" @mousemove="mover" @mouseleave="hover = null; desenhar()"></canvas>
          <div v-if="hover" class="leitura">t = {{ hover.t.toFixed(2) }} · Z(t) = {{ hover.z.toFixed(4) }}</div>
        </div>
        <table class="zeros">
          <thead><tr><th>n</th><th>calculado</th><th>valor conhecido</th><th>erro</th></tr></thead>
          <tbody>
            <tr v-for="r in tabela" :key="r.n">
              <td>{{ r.n }}</td>
              <td>{{ r.calc.toFixed(6) }}</td>
              <td>{{ r.real ? r.real.toFixed(6) : '—' }}</td>
              <td>{{ r.erro !== null ? r.erro.toFixed(4) : '—' }}</td>
            </tr>
          </tbody>
        </table>
        <p class="nota-rodape">Fórmula de Riemann-Siegel com o primeiro termo de correção. O erro é maior para t pequeno e diminui conforme t cresce.</p>
      </section>

      <footer>
        <p>A Hipótese de Riemann continua em aberto. Os resultados de 2026 são avanços parciais, verificados por especialistas e por prova formal.</p>
      </footer>
    </main>
    <main v-else class="espera"><p>Carregando o caderno… o backend está rodando na porta 3000?</p></main>
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
.indice { position: sticky; top: 0; height: 100vh; padding: 40px 28px; border-right: 1px solid var(--linha); display: flex; flex-direction: column; }
.marca { font-family: var(--serif); font-size: 34px; font-style: italic; margin-bottom: 40px; }
.indice nav { display: flex; flex-direction: column; gap: 4px; }
.indice a { cursor: pointer; font-family: var(--serif); font-size: 15px; color: var(--apagado); padding: 6px 0; display: flex; gap: 12px; transition: color .2s; }
.indice a:hover { color: var(--tinta); }
.indice a.on { color: var(--vermelho); }
.indice .num { font-family: var(--mono); font-size: 11px; width: 22px; padding-top: 3px; }
.rodape-indice { margin-top: auto; font-family: var(--mono); font-size: 11px; color: var(--apagado); line-height: 1.6; }

main { padding: 0 72px 80px; max-width: 1080px; }
section { padding: 96px 0 40px; border-bottom: 1px solid var(--linha); }
.cab { display: flex; align-items: baseline; gap: 20px; margin-bottom: 32px; }
.romano { font-family: var(--serif); font-style: italic; font-size: 20px; color: var(--vermelho); }
h1, h2, h3 { font-family: var(--serif); font-weight: 400; margin: 0; }
h2 { font-size: 42px; line-height: 1.1; letter-spacing: -0.01em; }
h3 { font-size: 21px; font-weight: 500; }
.intro { font-size: 21px; color: var(--tinta-2); max-width: 680px; margin: 0 0 40px; }

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

.fichas { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 0; border-top: 1px solid var(--linha); border-left: 1px solid var(--linha); }
.fichas article { padding: 24px; border-right: 1px solid var(--linha); border-bottom: 1px solid var(--linha); transition: background .2s; }
.fichas article:hover { background: var(--papel-2); }
.mono { width: 48px; height: 48px; border-radius: 50%; border: 1px solid var(--tinta); display: flex; align-items: center; justify-content: center; font-family: var(--serif); font-style: italic; font-size: 18px; margin-bottom: 16px; }
.fichas .anos { font-family: var(--mono); font-size: 12px; color: var(--vermelho); margin: 4px 0 10px; }
.fichas p { font-size: 16px; color: var(--tinta-2); margin: 0; }

.controle { display: flex; align-items: center; gap: 20px; margin-bottom: 20px; font-family: var(--mono); font-size: 13px; }
.controle input { flex: 1; accent-color: var(--vermelho); }
.status { color: var(--vermelho); min-width: 90px; text-align: right; }
.grafico { position: relative; border-top: 1px solid var(--tinta); border-bottom: 1px solid var(--tinta); }
.grafico canvas { width: 100%; height: 340px; display: block; cursor: crosshair; }
.leitura { position: absolute; top: 10px; right: 10px; font-family: var(--mono); font-size: 12px; background: var(--papel); padding: 4px 8px; border: 1px solid var(--linha); }
.zeros { width: 100%; border-collapse: collapse; margin-top: 32px; font-family: var(--mono); font-size: 14px; }
.zeros th { text-align: left; font-weight: 400; color: var(--apagado); font-size: 11px; text-transform: uppercase; letter-spacing: .06em; padding: 8px 0; border-bottom: 1px solid var(--tinta); }
.zeros td { padding: 8px 0; border-bottom: 1px solid var(--linha); }
.zeros td:last-child { color: var(--vermelho); }
.nota-rodape { font-size: 15px; font-style: italic; color: var(--apagado); }

footer { padding: 56px 0; font-style: italic; color: var(--apagado); }
.espera { display: flex; align-items: center; min-height: 100vh; font-style: italic; color: var(--apagado); }

@media (max-width: 900px) {
  .pagina { grid-template-columns: 1fr; }
  .indice { display: none; }
  main { padding: 0 24px 60px; }
  .destaque { margin: 0 -24px; padding: 72px 24px; }
  .duas, .colunas { grid-template-columns: 1fr; columns: 1; }
  .faixa { position: static; }
  .selos, .fatos { grid-template-columns: 1fr 1fr; }
  h2 { font-size: 32px; }
}
</style>
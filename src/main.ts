import { invoke } from '@tauri-apps/api';
import * as Plotly from 'plotly.js-dist';


const plotLayout = {
  margin: { t: 0 },
  yaxis: {
    range: [-1, 1]
  }
};

const dynamicsLayout = {
  margin: { t: 0 },
  yaxis: {
    range: [0, 1]
  }
};

window.addEventListener('DOMContentLoaded', async () => {
  Plotly.newPlot('plot', await getData(0, 1, 1), plotLayout);
  Plotly.newPlot('simulation', [await getDynamicsData(0.7, 100, 1)], dynamicsLayout);

  document.getElementById('rParam')?.addEventListener('change', async e => {
    const r = +(e.target as any).value;
    document.getElementById('rValue')!.innerText = `${r}`;
    Plotly.newPlot('plot', await getData(0, 1, r), plotLayout);
    Plotly.newPlot('simulation', [await getDynamicsData(0.3, 50, r)], dynamicsLayout);
  });
});

async function getData(a: number, b: number, r: number)
  : Promise<{ x: number[], y: number[] }[]> {
  const [x, ...values] = await invoke(
    'plot_logistic_map',
    { a, b, delta: 0.001, r }
  ) as [number[], [...number[]]];
  return values.map((y, i) => ({ x, y, name: `f^${i + 1}(x)-x` }));
}

async function getDynamicsData(x0: number, n: number, r: number)
  : Promise<{ x: number[], y: number[] }> {
  const [x, y] = await invoke(
    'simulate_dynamics',
    { x0, n, r }
  ) as [number[], number[]];
  return { x, y };
}



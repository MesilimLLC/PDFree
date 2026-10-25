# Forms performance comparison

Twenty warm fresh-process runs used `tests/corpus/office.pdf`, identical PDF SHA-256, the repository benchmark runner, default Winit/FemtoVG request, and the same page/zoom interaction schedule. The previous reader-tools and page-tools reports provide two prior samples. Current run uses the packaged optimized forms build.

| Build/sample | First pixels p50 / p95 ms | Window request p50 / p95 ms | Tile render p50 / p95 ms | Peak working set p50 MiB |
|---|---:|---:|---:|---:|
| Reader Tools (Oct 9 AM) | 249.3 / 390.5 | 447.3 / 597.4 | 1.0 / 1.6 | 148.4 |
| Page Tools (Oct 9 PM) | 473.3 / 993.3 | 824.6 / 1443.9 | 1.0 / 1.9 | 138.3 |
| Forms (Oct 25) | 355.5 / 434.1 | 302.1 / 353.8 | 1.1 / 1.7 | 152.9 |

Against the most recent prior page-tools sample, current first-pixel publication changed by -24.9% at p50 (473.3 to 355.5 ms) and -56.3% at nearest-rank p95 (993.3 to 434.1 ms). Window-show request changed by -63.4% at p50 (824.6 to 302.1 ms). Tile-render p50 is close (0.98 vs 1.05 ms); p95 is 1.92 vs 1.70 ms. Peak working-set p50 is 152.9 MiB; the previous 135.1–151.2 MiB range is lower. Exact samples and executable hashes are in forms-performance-comparison.json and forms-warm-office.json.

The earlier reader-tools build was faster at first-pixel p50 (249.3 ms) but had 390.5 ms p95; current lands between the two prior releases at p50 and has a higher p95 than reader-tools. The differing prior results show substantial run-to-run/build variability. Across reports, CPU, GPU model, OS build, document bytes, runner, and requested renderer match. Current GPU driver is 32.0.16.2101 vs 32.0.16.1714, and current run followed a different reboot. Background workload was not controlled. These are observations; they do not isolate a forms-related cost or establish a causal speedup/regression.

A separate 20-trial empty-window run recorded window-show p50/p95 of 315.3/375.9 ms and peak working-set p50/p95 of 81.2/82.0 MiB. The original empty FemtoVG baseline was 1,000.6/1,528.8 ms and 66.2/77.8 MiB. This comparison spans different executables, GPU drivers, and boot sessions; treat it as historical context, not a controlled speedup.

These metrics measure image publication and a window-show request, not physical presentation. Frame gaps contain idle time and do not qualify scrolling. No post-reboot cold benchmark was run.

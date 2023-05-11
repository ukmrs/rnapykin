# Python bindings for [rnapkin!](https://github.com/ukmrs/rnapkin) :snake: :crab:

Rnapkin is a fast, convenient CLI utility for drawing RNAs that produces stylish SVGs.
These Python bindings provide an easy way to use rnapkin's functionality directly from your Python scripts.

This is an early implementation that currently only exposes the *rna2svg* function. 
It takes an RNA fold in **rnapkin** compliant format and returns the corresponding
SVG as a string.

## Installation

```
pip install rnapykin
```

## Quick Example

```python
from rnapykin import rna2svg

gorgeous_rna = """
    UUAUAGGCGAUGGAGUUCGCCAUAAACGCUGCUUAGCUAAUGACUCCUACCAGUAUCACUACUGGUAGGAGUCUAUUUUUUU
    .....(((((......)))))......(((....)))....((((((((((((((....)))))))))))))).........
    """

# rna / theme / rotation in degrees / background opacity / horizontal flip / vertical flip
svg = rna2svg(gorgeous_rna, "dark", 0, 1., False, False)

# do with it what you want. Save it, embed it in HTML, whatevs c:
with open("gorgeous_rna.svg", "w") as f:
    f.write(svg)
```

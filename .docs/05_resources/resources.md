# Instinct — the specialist tier

Instinct is the idea that upgrades Reflex without replacing it: when the
free engine abstains, a specialist trained for exactly that domain scores
the survivors on top, never instead.

## The idea vs the product

Reflex is a product — the free modelless engine that runs on your machine.
Instinct is the idea layered on top of it: the composition itself, taught
here by an open lane that embodies it. The lane is public — the GitHub link
below is live (one repo, single source of truth since the same-day owner
retirement of the internal split). The same idea runs one rung
deeper in Rethink, where a bag-specialist is too coarse and a trained
encoder head thinks behind HOSTED-ONLY (weights that never leave our
servers — you call, we think) serving; see the family resources page
(`/resources/` on reflex.gist.rs) for the full three-name picture.

## How the composition works

Reflex always answers first, free. Its fused gate (the confidence check
that decides whether to consult a specialist) returns a confident answer
in the µs-class and the specialist is never paid. On abstain — the engine
says "I don't know" instead of guessing — the survivors are pruned to the
top candidates and a sealed per-domain specialist scores them on top. The
answer carries a receipt: an audit trail of what answered, from which
lane, committed hash-style in both directions.

## The winner law

A specialist serves only if it beats the free engine on a frozen test
read — the GOAT gate. It is registered before it serves, and registration
refuses anything that does not strictly beat Reflex; a tie or a loss sells
nothing, and the free engine keeps answering. Losers are demoted, never
blended — selection is an atomic hot-swap of whole sealed artifacts, so a
decision always observes one whole server, never a mix.

## Where it runs

CPU hosts: the serving binary is one static artifact for a self-hosted
server, shipped in the cf-container tier. After the open, demo vessels
answer through a hosted API too — the specialists stay sealed artifacts,
and nothing is re-fitted at serve time.

## Links

- GitHub: [gist-rs/riir-instinct](https://github.com/gist-rs/riir-instinct)
  — **LIVE since 2026-10-03** — the one repo, single source of truth:
  public, full history (the owner retired the fresh-root/internal split
  the same day it landed).
- The measured side: [/bench/#instinct](/bench/#instinct) — every verdict
  on this page is a link there, never a typed number.

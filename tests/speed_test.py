from rust_vdp import rk4
import simulator
import timeit

import numpy as np


def _test_odeint():
    u = 0
    mu = 1.
    x = (0.1, 2.)
    t_span = np.linspace(0, 1, 10)
    out = simulator.rk4(simulator.vdp, t_span, x, u, mu)
    return out

def _test_odeint_rust():
    u = 0
    mu = 1.
    x = (0.1, 2.)
    t_span = list(np.linspace(0, 1, 10))
    out = rk4(t_span, x, u, mu)
    return out

def _timeit_ode():
    number = 10
    experiments = 100
    out = timeit.repeat(_test_odeint, number=number, repeat=experiments)
    f1 = np.array(out)
    print(f"n:{number} mu:{f1.mean():.6f}\tstd:{f1.std():.4f}")
    out = timeit.repeat(_test_odeint_rust, number=number, repeat=experiments)
    f2 = np.array(out)
    print(f"n:{number} mu:{f2.mean():.6f}\tstd:{f2.std():.4f}")



if __name__ =="__main__":
    _timeit_ode()

import os
import warnings

import cartopy.crs as ccrs
import matplotlib.pyplot as plt
import numpy as np
import pyart
from pyart.testing import get_test_data

warnings.filterwarnings("ignore")

RADAR_FILE = "examples/KDMX20240521_215236_V06"

radar = pyart.io.read_nexrad_archive(RADAR_FILE)
velocity_sweep = 1

velocity_dealiased = pyart.correct.dealias_region_based(
    radar,
    vel_field="velocity",
    centered=True,
)

# Add our data dictionary to the radar object
radar.add_field("corrected_velocity", velocity_dealiased, replace_existing=True)

fig = plt.figure(figsize=[8, 10])
ax = plt.subplot(211, projection=ccrs.PlateCarree())
display = pyart.graph.RadarMapDisplay(radar)
display.plot_ppi_map(
    "corrected_velocity",
    ax=ax,
    sweep=velocity_sweep,
    resolution="50m",
    vmin=-100,
    vmax=100,
    max_lat=42.274039,
    max_lon=-93.957085,
    min_lat=41.491023,
    min_lon=-92.977209,
    projection=ccrs.PlateCarree(),
    colorbar_label="Radial Velocity (m/s)",
    cmap="pyart_RRate11",
)

ax2 = plt.subplot(212, projection=ccrs.PlateCarree())
display = pyart.graph.RadarMapDisplay(radar)
display.plot_ppi_map(
    "velocity",
    ax=ax2,
    sweep=velocity_sweep,
    resolution="50m",
    vmin=-100,
    vmax=100,
    max_lat=42.274039,
    max_lon=-93.957085,
    min_lat=41.491023,
    min_lon=-92.977209,
    projection=ccrs.PlateCarree(),
    cmap="pyart_RRate11",
)
plt.show()

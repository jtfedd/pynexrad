import warnings

import cartopy.crs as ccrs
import matplotlib.pyplot as plt
import pyart
from unravel import dealias

warnings.filterwarnings("ignore")

RADAR_FILE = "examples/KDMX20240521_215236_V06"

print("Loading...")

radar = pyart.io.read_nexrad_archive(RADAR_FILE)
velocity_sweep = 1

nyquist = []
for i in range(radar.nsweeps):
    nyquist.append(radar.get_nyquist_vel(i))

print(nyquist)

print("Dealiasing (unravel)...")

velocity_unravelled = dealias.unravel_3D_pyart(
    radar, velname="velocity", dbzname="reflectivity", nyquist_velocity=nyquist
)

vel_meta = pyart.config.get_metadata("velocity")
vel_meta["data"] = velocity_unravelled
vel_meta["_FillValue"] = -9999
vel_meta["comment"] = (
    "Corrected using the UNRAVEL algorithm developed by Louf et al. (2020) doi:10.1175/jtech-d-19-0020.1 available at https://github.com/vlouf/dealias"
)
vel_meta["long_name"] = "Doppler radial velocity of scatterers away from instrument"
vel_meta["units"] = "m s-1"

# Add our data dictionary to the radar object
radar.add_field("corrected_velocity", vel_meta, replace_existing=True)

print("Building plots")

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

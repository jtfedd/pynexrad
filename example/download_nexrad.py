import pynexrad

RADAR_BUCKET = "noaa-nexrad-level2"
KEY = "2022/03/05/KDMX/KDMX20220305_233003_V06"

level2File = pynexrad.download_nexrad_file(
    pynexrad.FileMetadata("KDMX", 2022, 3, 5, "KDMX20220305_233003_V06")
)

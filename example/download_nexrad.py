import pynexrad

level2File = pynexrad.download_nexrad_file(
    "KDMX", 2022, 3, 5, "KDMX20220305_233003_V06"
)

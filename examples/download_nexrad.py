import pynexrad

print(pynexrad.list_records("KDMX", 2022, 3, 5))

level2File = pynexrad.download_nexrad_file(
    "KDMX20220305_233003_V06"
)

print("REF", len(level2File.reflectivity))
print("VEL", len(level2File.velocity))

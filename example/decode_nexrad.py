import boto3
import botocore

import pynexrad

RADAR_BUCKET = "noaa-nexrad-level2"
KEY = "2022/03/05/KDMX/KDMX20220305_233003_V06"

print("reading from s3")

config = botocore.client.Config(
    signature_version=botocore.UNSIGNED, user_agent_extra="Resource"
)

bucket = boto3.resource("s3", config=config).Bucket(RADAR_BUCKET)
client = boto3.client("s3", config=config)

obj = client.get_object(Bucket=RADAR_BUCKET, Key=KEY)
data = obj["Body"].read()

print("parsing")

level2File = pynexrad.parse_nexrad_file(data)

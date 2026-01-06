"""
Integration tests for the pynexrad module
"""
from typing import List
import unittest
from concurrent.futures import ThreadPoolExecutor

from pynexrad import (
    PyChunk,
    PySweep,
    PyLevel2File,
    convert_chunks,
    download_chunk,
    get_latest_volume,
    list_chunks_in_volume,
    list_records,
    download_nexrad_file,
)


EXPECTED_RECORDS = [
    'KDMX20220305_000146_V06',
    'KDMX20220305_000507_V06',
    'KDMX20220305_000828_V06',
    'KDMX20220305_001149_V06',
    'KDMX20220305_001510_V06',
    'KDMX20220305_001830_V06',
    'KDMX20220305_002151_V06',
    'KDMX20220305_002512_V06',
    'KDMX20220305_002833_V06',
    'KDMX20220305_003154_V06',
    'KDMX20220305_003515_V06',
    'KDMX20220305_003902_V06',
    'KDMX20220305_004223_V06',
    'KDMX20220305_004544_V06',
    'KDMX20220305_004905_V06',
    'KDMX20220305_005225_V06',
    'KDMX20220305_005559_V06',
    'KDMX20220305_005946_V06',
    'KDMX20220305_005946_V06_MDM',
    'KDMX20220305_010347_V06',
    'KDMX20220305_010747_V06',
    'KDMX20220305_011201_V06',
    'KDMX20220305_011601_V06',
    'KDMX20220305_012015_V06',
    'KDMX20220305_012442_V06',
    'KDMX20220305_012854_V06',
    'KDMX20220305_013325_V06',
    'KDMX20220305_013756_V06',
    'KDMX20220305_014220_V06',
    'KDMX20220305_014658_V06',
    'KDMX20220305_015131_V06',
    'KDMX20220305_015558_V06',
    'KDMX20220305_015558_V06_MDM',
    'KDMX20220305_020024_V06',
    'KDMX20220305_020452_V06',
    'KDMX20220305_020920_V06',
    'KDMX20220305_021347_V06',
    'KDMX20220305_021927_V06',
    'KDMX20220305_022350_V06',
    'KDMX20220305_022817_V06',
    'KDMX20220305_023239_V06',
    'KDMX20220305_023702_V06',
    'KDMX20220305_024124_V06',
    'KDMX20220305_024548_V06',
    'KDMX20220305_025010_V06',
    'KDMX20220305_025437_V06',
    'KDMX20220305_025904_V06',
    'KDMX20220305_025904_V06_MDM',
    'KDMX20220305_030331_V06',
    'KDMX20220305_030803_V06',
    'KDMX20220305_031235_V06',
    'KDMX20220305_031720_V06',
    'KDMX20220305_032205_V06',
    'KDMX20220305_032650_V06',
    'KDMX20220305_033134_V06',
    'KDMX20220305_033619_V06',
    'KDMX20220305_034047_V06',
    'KDMX20220305_034515_V06',
    'KDMX20220305_034953_V06',
    'KDMX20220305_035431_V06',
    'KDMX20220305_035909_V06',
    'KDMX20220305_035909_V06_MDM',
    'KDMX20220305_040348_V06',
    'KDMX20220305_040819_V06',
    'KDMX20220305_041252_V06',
    'KDMX20220305_041710_V06',
    'KDMX20220305_042134_V06',
    'KDMX20220305_042558_V06',
    'KDMX20220305_043035_V06',
    'KDMX20220305_043502_V06',
    'KDMX20220305_043916_V06',
    'KDMX20220305_044325_V06',
    'KDMX20220305_044739_V06',
    'KDMX20220305_045153_V06',
    'KDMX20220305_045607_V06',
    'KDMX20220305_045607_V06_MDM',
    'KDMX20220305_050020_V06',
    'KDMX20220305_050433_V06',
    'KDMX20220305_050846_V06',
    'KDMX20220305_051300_V06',
    'KDMX20220305_051714_V06',
    'KDMX20220305_052127_V06',
    'KDMX20220305_052541_V06',
    'KDMX20220305_052955_V06',
    'KDMX20220305_053355_V06',
    'KDMX20220305_053755_V06',
    'KDMX20220305_054155_V06',
    'KDMX20220305_054555_V06',
    'KDMX20220305_054955_V06',
    'KDMX20220305_055355_V06',
    'KDMX20220305_055754_V06',
    'KDMX20220305_055754_V06_MDM',
    'KDMX20220305_060155_V06',
    'KDMX20220305_060554_V06',
    'KDMX20220305_060954_V06',
    'KDMX20220305_061341_V06',
    'KDMX20220305_061728_V06',
    'KDMX20220305_062133_V06',
    'KDMX20220305_062547_V06',
    'KDMX20220305_063001_V06',
    'KDMX20220305_063427_V06',
    'KDMX20220305_063859_V06',
    'KDMX20220305_064326_V06',
    'KDMX20220305_064753_V06',
    'KDMX20220305_065221_V06',
    'KDMX20220305_065659_V06',
    'KDMX20220305_065659_V06_MDM',
    'KDMX20220305_070137_V06',
    'KDMX20220305_070621_V06',
    'KDMX20220305_071106_V06',
    'KDMX20220305_071552_V06',
    'KDMX20220305_072037_V06',
    'KDMX20220305_072515_V06',
    'KDMX20220305_072953_V06',
    'KDMX20220305_073424_V06',
    'KDMX20220305_073857_V06',
    'KDMX20220305_074342_V06',
    'KDMX20220305_074814_V06',
    'KDMX20220305_075258_V06',
    'KDMX20220305_075743_V06',
    'KDMX20220305_075743_V06_MDM',
    'KDMX20220305_080210_V06',
    'KDMX20220305_080655_V06',
    'KDMX20220305_081140_V06',
    'KDMX20220305_081624_V06',
    'KDMX20220305_082056_V06',
    'KDMX20220305_082528_V06',
    'KDMX20220305_083001_V06',
    'KDMX20220305_083432_V06',
    'KDMX20220305_083910_V06',
    'KDMX20220305_084347_V06',
    'KDMX20220305_084825_V06',
    'KDMX20220305_085302_V06',
    'KDMX20220305_085733_V06',
    'KDMX20220305_085733_V06_MDM',
    'KDMX20220305_090203_V06',
    'KDMX20220305_090634_V06',
    'KDMX20220305_091052_V06',
    'KDMX20220305_091510_V06',
    'KDMX20220305_091915_V06',
    'KDMX20220305_092319_V06',
    'KDMX20220305_092723_V06',
    'KDMX20220305_093113_V06',
    'KDMX20220305_093504_V06',
    'KDMX20220305_093856_V06',
    'KDMX20220305_094225_V06',
    'KDMX20220305_094607_V06',
    'KDMX20220305_094949_V06',
    'KDMX20220305_095354_V06',
    'KDMX20220305_095740_V06',
    'KDMX20220305_095740_V06_MDM',
    'KDMX20220305_100156_V06',
    'KDMX20220305_100627_V06',
    'KDMX20220305_101111_V06',
    'KDMX20220305_101556_V06',
    'KDMX20220305_102152_V06',
    'KDMX20220305_102629_V06',
    'KDMX20220305_103056_V06',
    'KDMX20220305_103524_V06',
    'KDMX20220305_104100_V06',
    'KDMX20220305_104700_V06',
    'KDMX20220305_105313_V06',
    'KDMX20220305_110007_V06',
    'KDMX20220305_110007_V06_MDM',
    'KDMX20220305_110716_V06',
    'KDMX20220305_111424_V06',
    'KDMX20220305_112119_V06',
    'KDMX20220305_112827_V06',
    'KDMX20220305_113452_V06',
    'KDMX20220305_114028_V06',
    'KDMX20220305_114611_V06',
    'KDMX20220305_115222_V06',
    'KDMX20220305_115834_V06',
    'KDMX20220305_115834_V06_MDM',
    'KDMX20220305_120434_V06',
    'KDMX20220305_121018_V06',
    'KDMX20220305_121601_V06',
    'KDMX20220305_122152_V06',
    'KDMX20220305_122744_V06',
    'KDMX20220305_123345_V06',
    'KDMX20220305_123830_V06',
    'KDMX20220305_124315_V06',
    'KDMX20220305_124800_V06',
    'KDMX20220305_125244_V06',
    'KDMX20220305_125729_V06',
    'KDMX20220305_125729_V06_MDM',
    'KDMX20220305_130151_V06',
    'KDMX20220305_130614_V06',
    'KDMX20220305_131037_V06',
    'KDMX20220305_131459_V06',
    'KDMX20220305_131921_V06',
    'KDMX20220305_132345_V06',
    'KDMX20220305_132756_V06',
    'KDMX20220305_133206_V06',
    'KDMX20220305_133623_V06',
    'KDMX20220305_134041_V06',
    'KDMX20220305_134451_V06',
    'KDMX20220305_134901_V06',
    'KDMX20220305_135258_V06',
    'KDMX20220305_135655_V06',
    'KDMX20220305_135655_V06_MDM',
    'KDMX20220305_140047_V06',
    'KDMX20220305_140426_V06',
    'KDMX20220305_140805_V06',
    'KDMX20220305_141138_V06',
    'KDMX20220305_141503_V06',
    'KDMX20220305_141823_V06',
    'KDMX20220305_142144_V06',
    'KDMX20220305_142505_V06',
    'KDMX20220305_142821_V06',
    'KDMX20220305_143137_V06',
    'KDMX20220305_143453_V06',
    'KDMX20220305_143809_V06',
    'KDMX20220305_144125_V06',
    'KDMX20220305_144454_V06',
    'KDMX20220305_144809_V06',
    'KDMX20220305_145147_V06',
    'KDMX20220305_145521_V06',
    'KDMX20220305_145907_V06',
    'KDMX20220305_145907_V06_MDM',
    'KDMX20220305_150249_V06',
    'KDMX20220305_150646_V06',
    'KDMX20220305_151051_V06',
    'KDMX20220305_151509_V06',
    'KDMX20220305_151908_V06',
    'KDMX20220305_152321_V06',
    'KDMX20220305_152748_V06',
    'KDMX20220305_153215_V06',
    'KDMX20220305_153643_V06',
    'KDMX20220305_154106_V06',
    'KDMX20220305_154528_V06',
    'KDMX20220305_154950_V06',
    'KDMX20220305_155413_V06',
    'KDMX20220305_155834_V06',
    'KDMX20220305_155834_V06_MDM',
    'KDMX20220305_160256_V06',
    'KDMX20220305_160719_V06',
    'KDMX20220305_161141_V06',
    'KDMX20220305_161603_V06',
    'KDMX20220305_162025_V06',
    'KDMX20220305_162447_V06',
    'KDMX20220305_162909_V06',
    'KDMX20220305_163354_V06',
    'KDMX20220305_163826_V06',
    'KDMX20220305_164249_V06',
    'KDMX20220305_164712_V06',
    'KDMX20220305_165149_V06',
    'KDMX20220305_165627_V06',
    'KDMX20220305_165627_V06_MDM',
    'KDMX20220305_170111_V06',
    'KDMX20220305_170556_V06',
    'KDMX20220305_171018_V06',
    'KDMX20220305_171504_V06',
    'KDMX20220305_171941_V06',
    'KDMX20220305_172403_V06',
    'KDMX20220305_172824_V06',
    'KDMX20220305_173238_V06',
    'KDMX20220305_173634_V06',
    'KDMX20220305_174016_V06',
    'KDMX20220305_174421_V06',
    'KDMX20220305_174812_V06',
    'KDMX20220305_175145_V06',
    'KDMX20220305_175518_V06',
    'KDMX20220305_175922_V06',
    'KDMX20220305_175922_V06_MDM',
    'KDMX20220305_180340_V06',
    'KDMX20220305_180811_V06',
    'KDMX20220305_181256_V06',
    'KDMX20220305_181741_V06',
    'KDMX20220305_182343_V06',
    'KDMX20220305_182821_V06',
    'KDMX20220305_183252_V06',
    'KDMX20220305_183737_V06',
    'KDMX20220305_184204_V06',
    'KDMX20220305_184637_V06',
    'KDMX20220305_185105_V06',
    'KDMX20220305_185528_V06',
    'KDMX20220305_185951_V06',
    'KDMX20220305_185951_V06_MDM',
    'KDMX20220305_190413_V06',
    'KDMX20220305_190835_V06',
    'KDMX20220305_191257_V06',
    'KDMX20220305_191724_V06',
    'KDMX20220305_192152_V06',
    'KDMX20220305_192615_V06',
    'KDMX20220305_193036_V06',
    'KDMX20220305_193520_V06',
    'KDMX20220305_193942_V06',
    'KDMX20220305_194405_V06',
    'KDMX20220305_194828_V06',
    'KDMX20220305_195255_V06',
    'KDMX20220305_195733_V06',
    'KDMX20220305_195733_V06_MDM',
    'KDMX20220305_200210_V06',
    'KDMX20220305_200642_V06',
    'KDMX20220305_201114_V06',
    'KDMX20220305_201552_V06',
    'KDMX20220305_202037_V06',
    'KDMX20220305_202515_V06',
    'KDMX20220305_202945_V06',
    'KDMX20220305_203416_V06',
    'KDMX20220305_203847_V06',
    'KDMX20220305_204502_V06',
    'KDMX20220305_205116_V06',
    'KDMX20220305_205730_V06',
    'KDMX20220305_205730_V06_MDM',
    'KDMX20220305_210331_V06',
    'KDMX20220305_210849_V06',
    'KDMX20220305_211407_V06',
    'KDMX20220305_211955_V06',
    'KDMX20220305_212556_V06',
    'KDMX20220305_213210_V06',
    'KDMX20220305_213824_V06',
    'KDMX20220305_214438_V06',
    'KDMX20220305_215106_V06',
    'KDMX20220305_215748_V06',
    'KDMX20220305_215748_V06_MDM',
    'KDMX20220305_220420_V06',
    'KDMX20220305_221051_V06',
    'KDMX20220305_221723_V06',
    'KDMX20220305_222333_V06',
    'KDMX20220305_222943_V06',
    'KDMX20220305_223552_V06',
    'KDMX20220305_224202_V06',
    'KDMX20220305_224856_V06',
    'KDMX20220305_225550_V06',
    'KDMX20220305_225550_V06_MDM',
    'KDMX20220305_230243_V06',
    'KDMX20220305_230936_V06',
    'KDMX20220305_231630_V06',
    'KDMX20220305_232324_V06',
    'KDMX20220305_233003_V06',
    'KDMX20220305_233656_V06',
    'KDMX20220305_234350_V06',
    'KDMX20220305_235045_V06',
    'KDMX20220305_235739_V06',
    'KDMX20220305_235739_V06_MDM',
]


class TestPynexrad(unittest.TestCase):
    """
    Integration tests for the pynexrad module
    """
    def test_list_records(self) -> None:
        """
        Integration test to validate listing records for a given date
        """
        records = list_records("KDMX", 2022, 3, 5)

        self.assertListEqual(records, EXPECTED_RECORDS)

    def test_download_nexrad_file(self) -> None:
        """
        Integration test to validate downloading a nexrad level 2 volume file.

        This downloads the file and verifies some metadata about the result.
        """
        level_2_file = download_nexrad_file("KDMX20220305_233003_V06")

        self.assertIsInstance(level_2_file, PyLevel2File)
        self.assertEqual(len(level_2_file.reflectivity), 17)
        self.assertEqual(len(level_2_file.velocity), 17)

        self.assertIsInstance(level_2_file.reflectivity[0], PySweep)
        self.assertIsInstance(level_2_file.velocity[0], PySweep)

    def test_get_realtime_chunks(self) -> None:
        """
        Integration test to validate find the latest realtime data
        """
        latest_volume = get_latest_volume("KDMX")

        chunks_in_volume = list_chunks_in_volume("KDMX", latest_volume)
        self.assertGreater(len(chunks_in_volume), 0)

        for chunk in chunks_in_volume:
            self.assertEqual(chunk.site, "KDMX")
            self.assertEqual(chunk.volume, latest_volume)

        chunk_data = download_chunk(chunks_in_volume[-1])
        self.assertGreater(len(chunk_data.data), 0)

    def test_get_realtime_volume(self) -> None:
        """
        Integration test to validate that a complete realtime
        volume matches the archive data.
        """
        latest_volume = get_latest_volume("KDMX")
        previous_volume = latest_volume - 1
        if previous_volume <= 0:
            previous_volume = 99

        chunks_in_volume = list_chunks_in_volume("KDMX", previous_volume)
        self.assertGreater(len(chunks_in_volume), 0)

        chunk_data: List[PyChunk] = []
        with ThreadPoolExecutor() as executor:
            for result in executor.map(download_chunk, chunks_in_volume):
                chunk_data.append(result)

        volume = convert_chunks(chunk_data)

        name_parts = chunks_in_volume[0].name.split('-')
        date = name_parts[0]
        time = name_parts[1]

        archive_key = f'KDMX{date}_{time}_V06'
        archive_volume = download_nexrad_file(archive_key)

        assert_l2files_equal(self, volume, archive_volume)


def assert_l2files_equal(
    t: unittest.TestCase,
    a: PyLevel2File,
    b: PyLevel2File,
) -> None:
    """Asserts that two PyLevel2File instances are equal"""
    t.assertIsInstance(a, PyLevel2File)
    t.assertIsInstance(b, PyLevel2File)

    t.assertEqual(len(a.reflectivity), len(b.reflectivity))
    for i in range(len(a.reflectivity)):
        assert_sweeps_equal(t, a.reflectivity[i], b.reflectivity[i])

    t.assertEqual(len(a.velocity), len(b.velocity))
    for i in range(len(a.velocity)):
        assert_sweeps_equal(t, a.velocity[i], b.velocity[i])


def assert_sweeps_equal(
    t: unittest.TestCase,
    a: PySweep,
    b: PySweep,
) -> None:
    """Asserts that two PySweep instances are equal"""
    t.assertIsInstance(a, PySweep)
    t.assertIsInstance(b, PySweep)

    t.assertAlmostEqual(a.elevation, b.elevation, places=6)
    t.assertAlmostEqual(a.az_first, b.az_first, places=6)
    t.assertAlmostEqual(a.az_step, b.az_step, places=6)
    t.assertEqual(a.az_count, b.az_count)

    t.assertAlmostEqual(a.range_first, b.range_first, places=6)
    t.assertAlmostEqual(a.range_step, b.range_step, places=6)
    t.assertEqual(a.range_count, b.range_count)

    t.assertEqual(a.start_time, b.start_time)
    t.assertEqual(a.end_time, b.end_time)

    t.assertEqual(a.data, b.data)


if __name__ == '__main__':
    unittest.main()

CREATE DATABASE IF NOT EXISTS duckdb;
USE duckdb;
--
-- Table structure for table `Names`
--

CREATE TABLE IF NOT EXISTS `Names` (
  `DuckID` int(11) NOT NULL AUTO_INCREMENT,
  `name` char(32) NOT NULL,
  PRIMARY KEY (`DuckID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

--
-- Table structure for table `Habilities`
--

CREATE TABLE IF NOT EXISTS `Habilities` (
  `HabilityID` int(11) NOT NULL AUTO_INCREMENT,
  `DuckID` int(11) DEFAULT NULL,
  `Hability` char(128) DEFAULT NULL,
  PRIMARY KEY (`HabilityID`),
  KEY `DuckID` (`DuckID`),
  CONSTRAINT `1` FOREIGN KEY (`DuckID`) REFERENCES `Names` (`DuckID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

--
-- Table structure for table `Races`
--

CREATE TABLE IF NOT EXISTS `Races` (
  `RaceID` int(11) NOT NULL AUTO_INCREMENT,
  `DuckID` int(11) DEFAULT NULL,
  `race` char(32) NOT NULL,
  PRIMARY KEY (`RaceID`),
  KEY `DuckID` (`DuckID`),
  CONSTRAINT `1` FOREIGN KEY (`DuckID`) REFERENCES `Names` (`DuckID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

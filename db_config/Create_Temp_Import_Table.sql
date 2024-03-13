USE [Water]
GO

/****** Object:  Table [dbo].[TempImport]    Script Date: 11/10/2016 10:39:12 AM ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

CREATE TABLE [dbo].[TempImport](
	[AccountNo1] [nvarchar](50) NULL,
	[CyclNo1] [nvarchar](50) NULL,
	[Status] [int] NULL,
	[OwnerName] [nvarchar](50) NULL,
	[PropAddrKey] [nvarchar](50) NULL,
	[AddrLine2] [nvarchar](50) NULL,
	[MeterID] [nvarchar](50) NOT NULL,
	[CyclNo2] [int] NULL,
	[ReadDigits] [int] NULL,
	[No] [int] NULL,
	[Type] [nvarchar](50) NULL,
	[ARB] [nvarchar](50) NULL,
	[FileKey] [nvarchar](50) NULL,
	[StreetDirection] [nvarchar](50) NULL,
	[StreetName] [nvarchar](50) NULL,
	[StreetNumber] [nvarchar](50) NULL,
	[StreetUnit] [nvarchar](50) NULL,
	[MeterSerial] [nvarchar](50) NULL,
	[PrintKey] [nvarchar](50) NULL,
	[MeterSize] [numeric](18, 4) NULL,
	[Special] [nvarchar](25) NULL
) ON [PRIMARY]

GO


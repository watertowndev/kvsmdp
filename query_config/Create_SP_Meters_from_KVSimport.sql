USE [Water]
GO
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

CREATE OR ALTER PROCEDURE [dbo].[Meters_From_KVSimport]
	
AS
BEGIN
	BEGIN
		TRUNCATE TABLE dbo.TempImport;
	END
	IF @@ROWCOUNT = 0
    BEGIN
		BULK INSERT dbo.TempImport 
		FROM "C:\import_staging\WaterMetersProcessed.csv"
		WITH ( FIELDTERMINATOR = ',', ROWTERMINATOR = '0x0A' );
	END
	IF @@ROWCOUNT <> 0
	BEGIN
		MERGE dbo.Meters AS target
		USING dbo.TempImport AS source
		ON (target.MeterIDText = source.MeterID)
		WHEN MATCHED THEN 
			UPDATE SET target.OwnerName = source.OwnerName,
						target.StreetNumber = source.StreetNumber,
				
						target.StreetName = source.StreetName,
						target.PostDir = source.StreetDirection,
						target.AccountNo1 = source.AccountNo1,
						target.StreetUnit = source.StreetUnit,
						target.AddrLine2 = source.AddrLine2,
						target.CyclNo1 = source.CyclNo1,
						target.Status = source.Status,
						target.ARB = source.ARB,
						target.serialNum = source.MeterSerial,
						target.size = source.MeterSize,
						target.Special = source.Special,
						target.FullAddress = isnull(str(source.StreetNumber),'') +' '+ isnull(source.StreetName,'') + ' ' +  isnull(source.StreetDirection,'') + ' ' + isnull(source.AddrLine2,'') + ' ' + isnull(source.StreetUnit,''),
						target.last_edited_date = GETUTCDATE(),
						target.last_edited_user = 'KVSupdate'
		;
	END
END


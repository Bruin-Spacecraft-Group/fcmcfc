# Sensors List

### Acronyms
* CDH: Command and Data Handling: 
* DCIU: Digital Control Interface Unit
* EPS: Electric Power System
* ADCS: Attitude Determination Control System
* RTC: Real Time Clock

### Data Protocols

* I2C/SPI
	* communication to sensors (e.g. voltage)
* UART (serial)
	* debug interface, DCIU control, camera
* GPIO
	* watchdog heartbeat pulse, bit-banging
* RTC
	* button battery + crystal for ultra low-power timekeeping
* CAN
	* data from comms module
* ETH
	* ping from comms module
* PWM
	* control for attitude actuators
* USB Serial
	* development, debugging

### From system breakdown slide
* CDH needs to recieve telemetry from:
	* DCIU
	* Thermal Sensors
	* Transciever
	* EPS
	* ADCS
	* Structures
* CDH needs to send commands to:
	* DCIU
	* Transciever
	* EPS
	* ADCS
	* Structures

### Components Potential Options
* CDH
	* Flight Computer:
		* Potential Options (Will we be using Beaglebone?)
			* Beaglebone Black
			* Raspberry Pi Zero
			* Teensy 3.6
			* Pumpkin
	* RTC
		* Potential Options: (What will we be using?)
	* Differential Amplifier
		* Potential Options: (What will we be using?)

* ADCS
	* Reaction Wheels
		* Potential Options: (Which motor controller are we using? Will it be PWM?)
	* Magnetorquers
		* Potential Options (Which magnetorquer are we using?)
			* CubeTorquer
				* Nominal magnetic moment (2.5V @ 25 degC): +/- 0.24 Am2
				* Residual Moment: <0.48 mAm2
			* CubeCoil
				* Nominal magnetic moment (5V @ 25 degC): +/- 0.13 Am2
	* Camera
		* 4D Systems uCAM-III (Are we using this?)
			* Very good documentation
			* Auto WB, brightness, contrast; manual focus
			* Operate under -20C ~ 85C
			* Serial
			* Adjustable lens -> 115deg
	* IMU: Inertial Measurement Unit
		* CHRobotics UM7 Sensor (Are we using this?)
			* On-board Kalman Filtering
			* Outputs include euler angles, attitude quarternion
			* Raw data from Accelerometer, Gyroscope and Magnetometer
			* Magnetometer serves as external reference

* EPS
	* Potential Options: (Which one are we using, and will there be a builtin controller, or will we use seperate senors?)
		* Pumpkin BM2 Battery Module
		* EXA BA0x Pegasus Class BA01/D
		* Panasonic NCR18650 (x6)

* Thermal Sensors
	* Potential Options: (What will we be using?)

* Communications Transciever:
	* Potential Options: (Which one will we be using?)
		* LimeSDR for data, functionality, and prototyping
		* Nanoavionics S-Band Transciever

* DCIU:
	* UART connection
	* CDH simply commands "Startup", "Throttle Level 1", "Shutdown" etc.
	* Contains entire logic structure, calibration data, and throttle table for MiXI
	* (Is there a specification of the commands we can send?)

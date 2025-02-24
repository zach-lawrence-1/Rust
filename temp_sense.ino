#include <SoftwareSerial.h>
#include <LiquidCrystal.h>
#include <Keypad.h>
#include <Wire.h>
#include <AHT20.h>
#include <TimeLib.h>

//temperature
AHT20 aht;
float ACTemp = 20.5;
float comfortTemp = 21;
bool celcius = true;
bool changingTemp = false;

enum modes
{
    LOCKED, UNLOCKED, NEWPASS, TEMP, SETTEMP
};

//lcd stuff, analog pins
const int rs = A0, en = A1, d4 = A2, d5 = A3, d6 = 10, d7 = 11;
LiquidCrystal lcd(rs, en, d4, d5, d6, d7);

//password stuff
int mode = NEWPASS;
char password[5] = {'1', '2', '3', '4', '\0'};
char passEntry[5];
char tempEntry[6];
int digit = 0;
bool isUnlocked = false;
bool newPassword = false;

//keypad stuff
char keypadArray[4][4] = 
{
    {'1', '2', '3', 'A'},
    {'4', '5', '6', 'B'},
    {'7', '8', '9', 'C'},
    {'*', '0', '#', 'D'}
};
byte pinRows[4] = {9, 8, 7, 6};  //make sure that the pin data is not the same as the lcd pins
byte pinCols[4] = {5, 4, 3, 2};
Keypad keypad = Keypad(makeKeymap(keypadArray), pinRows, pinCols, 4, 4);

void setup()
{
    //lcd and serial output initialization
    Serial.begin(9600);
    lcd.begin(16, 2);
    lcd.clear();

    pinMode(12, OUTPUT);  //output pin for fan
    Wire.begin();  //initialize pins for temperature sensor
    
    if (aht.begin() == false)
    {
        Serial.println("AHT20 not detected. Please check wiring.");
        delay(2000);
        while(1);
    }

    Serial.println("AHT20 acknowledged.");
}

void loop()
{
    char key = keypad.getKey();

    switch (mode)
    {
        case LOCKED:
            lcd.setCursor(0, 0);
            lcd.print("Enter Password:");
            
            if (key)
            {
                //clear screen
                if (key == '*')
                {
                    digit = 0;
                    strcpy(passEntry, "");
                    lcd.clear();
                    return;
                }

                if (digit == 4 && key == '#')
                {
                    //check entered password against stored password and change state if needed
                    if (strcmp(passEntry, password) == 0)
                    {
                        digit = 0;
                        strcpy(passEntry, "");
                        mode = UNLOCKED;
                        lcd.clear();
                        return;
                    }

                    lcd.setCursor(0, 1);
                    lcd.print("wrong password");
                    delay(1000);
                    lcd.clear();
                    digit = 0;
                    strcpy(passEntry, "");
                }
                else if (digit < 4)
                {
                    if (key == '#')
                    {
                        return;
                    }

                    //allow what you entered on the keypad to be displayed
                    lcd.setCursor(digit, 1);
                    lcd.print(key);
                    passEntry[digit] = key;
                    digit++;
                }
            }
            break;
        case UNLOCKED:
            lcd.setCursor(0, 0);
            lcd.print("UNLOCKED");
            delay(1500);
            lcd.clear();
            mode = TEMP;

            if (changingTemp)
            {
                changingTemp = false;
                isUnlocked = true;
                mode = SETTEMP;
                return;
            }
            
            if (newPassword)
            {
                newPassword = false;
                strcpy(password, "");
                mode = NEWPASS;
            }
            
            break;
        case NEWPASS:
            //password already exists so we don't need to make a new one
            if (strlen(password) == 4)
            {
                lcd.clear();
                mode = TEMP;
                return;
            }

            lcd.setCursor(0, 0);
            lcd.print("put new password");

            if (key)
            {
                //clear screen
                if (key == '*')
                {
                    digit = 0;
                    strcpy(passEntry, "");
                    lcd.clear();
                    return;
                }

                //check if we typed in the correct length of password and store it
                if (digit == 4 && key == '#')
                {
                    delay(1000);
                    lcd.clear();
                    digit = 0;
                    strcpy(password, passEntry);
                    strcpy(passEntry, "");
                    mode = TEMP;
                }
                else if (digit < 4)
                {
                    if (key == '#')
                    {
                        return;
                    }
                    
                    //allow what you entered on the keypad to be displayed
                    lcd.setCursor(digit, 1);
                    lcd.print(key);
                    passEntry[digit] = key;
                    digit++;
                }
            }
            break;
        case TEMP:
            time_t timeInit;
            time_t timeFinal;
            isUnlocked = false;
            timeInit = now();
            timeFinal = now();

            lcd.setCursor(0, 0);
            lcd.print("Temp Ext:");
            lcd.setCursor(0,1);
            lcd.print("Set Temp:");

            /*
            for controlling the rate at which the temperature sensor is reading 
            and get consistent key input readings
            */
            while (key != 'A' && key != 'B' && key != 'C' && 
                ((second(timeFinal) - second(timeInit)) < 2) && 
                ((second(timeFinal) - second(timeInit)) >= 0))
            {
                key = keypad.getKey();
                timeFinal = now();
            }

            if (key == 'A')
            {
                celcius = !celcius;
            }
            else if (key == 'B')
            {
                digit = 0;
                mode = SETTEMP;
                lcd.clear();
                changingTemp = true;
                return;
            }
            else if (key == 'C')
            {
                newPassword = true;
                mode = LOCKED;
                lcd.clear();
                return;
            }

            if (aht.available() == true)
            {
                lcd.clear();
                Serial.println("time");
                Serial.println(now());
                
                float temperature = aht.getTemperature();
                char tempLetter[2] = {'C', '\0'};
                float tComf = comfortTemp;

                if (!celcius)
                {
                    temperature = (temperature * 9/5) + 32;
                    tComf = (tComf * 9/5) + 32;
                    tempLetter[0] = 'F';

                    if (temperature - tComf >= 5)
                    {
                        digitalWrite(12, HIGH);
                    }
                    else
                    {
                        digitalWrite(12, LOW);
                    }
                }
                else
                {
                    if (temperature - tComf >= 3)
                    {
                        digitalWrite(12, HIGH);
                    }
                    else
                    {
                        digitalWrite(12, LOW);
                    }
                }

                //Print the results
                lcd.setCursor(0, 0);
                lcd.print("Temp Ext:");
                lcd.setCursor(10, 0);
                lcd.print(temperature);
                lcd.setCursor(15, 0);
                lcd.print(tempLetter);

                lcd.setCursor(0,1);
                lcd.print("Set temp:");
                lcd.setCursor(10,1);
                lcd.print(tComf);
                lcd.print(tempLetter);
            }
            break;
        case SETTEMP:
            if (!isUnlocked)
            {
                mode = LOCKED;
                return;
            }

            lcd.setCursor(0, 0);
            lcd.print("Set desired temp");
            lcd.setCursor(digit, 1);

            if (celcius)
            {
                lcd.print("C");
            }
            else
            {
                lcd.print("F");
            }

            if (key)
            {
                //clear screen
                if (key == '*')
                {
                    digit = 0;
                    strcpy(tempEntry, "");
                    lcd.clear();
                    return;
                }

                //set comfort temp once it is entered on the keyboard
                if (digit == 5 && key == '#')
                {
                    lcd.clear();
                    lcd.setCursor(0, 0);
                    lcd.print("temp is set");
                    delay(1000);
                    lcd.clear();
                    digit = 0;
                    comfortTemp = atof(tempEntry);
                    strcpy(tempEntry, "");

                    if (!celcius)
                    {
                        comfortTemp = (comfortTemp - 32) * 5/9;
                    }

                    mode = TEMP;
                }
                else if (digit < 5)
                {
                    if (key == '#')
                    {
                        return;
                    }

                    if (key == 'D')
                    {
                        key = '.';
                    }

                    //allow what you entered on the keypad to be displayed
                    lcd.setCursor(digit, 1);
                    lcd.print(key);
                    tempEntry[digit] = key;
                    digit++;
                }
            }
            break;
        default:
            break;
    }
}

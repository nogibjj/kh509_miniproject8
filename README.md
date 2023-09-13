# Polars Descriptive Statistics Script
Mini-Project3
Katelyn Hucker (kh509)

My mini-project imports the iris.csv dataset and then calculates the descriptive statistics of petal length for iris flowers using the python polars library. 

# Changes 
_______________________________

### Requirements.txt:
The requirements file added polars 0.19.2 to its list of required libraries. I also added matplotlibb.pyplot 3.7.0. This allows polars and matplotlib.pyplot to be installed as packages. 

### Makefile:
The Makefile has been uncommented to actually run properly on the statistics script. The Makefile installs the libraries in the requirements.txt, tests the code with the test_main.py file, then formats and lints with python black and ruff. 

# New 
_______________________________

### polars_stat.py:
polars_stat.py is a statistics script that takes advantage of the polars library. The script imports a csv file as a dataframe, of the developers choosing, in this case it is the iris.csv dataset. The dataset contains varying numeric information about iris flowers. The whole dataset is imported with the read_csv method. The script contains a function that imports a dataframe's column, calculates and returns the descriptive stats of the column. The function is then called on the petal length coloumn of the iris data frame and printed out. The script also produces a boxplot of this user selected column of interest. The developer can change which column they are looking as the function allows new inputs. The figure is saved as a boxplot.png for that column of interest.The boxplot is a visualization that includes most of the descriptive statistics output, including median, max, min, and quantiles. The visualization for the petal length column is below.

![image](https://github.com/nogibjj/kh509_miniproject3/blob/6c2c558762002b185aa61adfc9fc938d9dee9b62/boxplot.png)


### test_main.py:
The test_main.py file is file used to to test the stat1.py script. It tests that the calculate descriptive stats function from the polars_stat.py script. The test function takes a local small dataframe column, "Height,' and asserts that the mean, min, max, and std is what should be outputted from the function. There is also a test case to confirm that a boxplot graph is formed and saved in the workflow.

# Running the project
_______________________________
The project uses the python template, from mini project 1, as a base. It was then edited and added on for this week's specific needs. The project builds through the github worflows folder, cicd.yml file. It installs, lints, tests, formats, and runs the python script. 

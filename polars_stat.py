import polars as pl
import matplotlib.pyplot as plt

df1 = pl.read_csv('iris.csv')

def calc_desc_stat(dataset_col):
    out=dataset_col.describe()
    return round(out,2)

print(calc_desc_stat(df1['petal.length']))
#print(df1.columns)
def boxplot_of_col(col):
    plt.boxplot(col)
    plt.show()
    plt.savefig("boxplot.png")


boxplot_of_col(df1,'petal.length')
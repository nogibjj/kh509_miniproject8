import polars as pl
import os
from polars_stat import calc_desc_stat, boxplot_of_col

data = {'Height': [5.1, 6.2, 5.1, 5.2]}

testing_df = pl.DataFrame(data)

def test_stats():
    out = calc_desc_stat(testing_df['Height'])
 #   print(out)
    mean_value = out.filter((pl.col('statistic') == 'mean'))['value'].to_list()[0]
 #   print(mean_value)
    mean_value = round(mean_value,1)
    assert mean_value == 5.4
    min_value = out.filter((pl.col('statistic') == 'min'))['value'].to_list()[0]
    assert min_value == 5.1
    max_value = out.filter((pl.col('statistic') == 'max'))['value'].to_list()[0]
    assert max_value == 6.2
    std_value = out.filter((pl.col('statistic') == 'std'))['value'].to_list()[0]
    std_value = round(std_value,2)
    assert std_value == .54

def does_graph_save():
    boxplot_of_col(testing_df['Height'])
    assert os.path.isfile("boxplot.png")

if __name__ == "__main__":
    test_stats()
    does_graph_save()
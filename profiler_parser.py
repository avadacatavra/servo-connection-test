#!/usr/bin/env python3

from bs4 import BeautifulSoup as bs
import json
import sys
import statistics
import math

#FIXME clean up repetition
def print_stats(category, times):
    mean = statistics.mean(times)
    median = statistics.median(times)
    stdev = statistics.stdev(times)
    minimum = min(times)
    maximum = max(times)
    total = sum(times)

    print("""%s (%f)
             Mean: \t%f
             Median: \t%f
             Std Dev: \t%f
             Min: \t%f
             Max: \t%f\n"""
            %(category, total, mean, median, stdev, minimum, maximum))

def print_net_stats(category, times, urls):
    mean = statistics.mean(times)
    median = statistics.median(times)
    stdev = statistics.stdev(times)
    minimum = min(times)
    maximum = max(times)
    total = sum(times)

    min_url = urls[times.index(minimum)]
    max_url = urls[times.index(maximum)]

    print("""%s (%f))
             Mean: \t%f
             Median: \t%f
             Std Dev: \t%f
             Min: \t%f (%s)
             Max: \t%f (%s)\n"""
            %(category, total, mean, median, stdev, minimum, min_url, maximum, max_url))



def main():
    if len(sys.argv) < 2:
        print("Please provide html file")
        sys.exit()

    soup = bs(open(sys.argv[1]), "html.parser")

    script = soup.findAll('script')[0].string
    data = script.split("window.TRACES = [",1)[-1].rsplit(';', 1)[0]

    time_profiles = {}
    urls = []

    #overall and total network timing
    first_net_time = math.inf
    last_net_time = 0
    first_time = math.inf
    last_time = 0


    for line in data.split(",\n")[:-1]:
        ele = json.loads(line)
        category = list(ele["category"].keys())[0]
        if not category in time_profiles:
            time_profiles[category] = []
  
        start = int(ele["startTime"])
        end = int(ele["endTime"])
        time = (end - start)/1000000        #in ms

        if start < first_time and start > 0:
            first_time = start
        if end > last_time:
            last_time = end

        #FIXME doesn't account for breaks when we aren't doing any net
        if category=="NetHTTPRequestResponse":          
            if start < first_net_time and start > 0:
                first_net_time = start
            if end > last_net_time:
                last_net_time = end
            try:
                url = ele["metadata"]["url"]
                urls.append(url)
            except TypeError:
                urls.append("Error")
     
        time_profiles[category].append(time)


    for category,times in time_profiles.items():
        if len(times) == 0:
            continue 
        if category == "NetHTTPRequestResponse":
            print_net_stats(category, times, urls)
        else:
            print_stats(category, times)

    #compute total times
    net_time = (last_net_time - first_net_time)/10000000
    print("Total time fetching resources: %f ms (%f s)"%(net_time, net_time/1000))
    print("Total resources fetched: %d"%len(time_profiles["NetHTTPRequestResponse"]))
    total_time = (last_time - first_time)/10000000
    print("Total time: %f ms (%f s)"%(total_time, total_time/1000))





if __name__ == "__main__":
    main()


def return_true():
    return True

def main():
    monthly_saving = 1
    year = 10
    inflation = 5 # percent
    salary_increase = 10 # percent
    investment_interest = 7 # percent

    inflation = inflation / 100
    salary_increase = salary_increase / 100 - inflation
    investment_interest = 0.07
    net_worth = 0

    for i in range(year):
        net_worth += (monthly_saving * 12) + (net_worth * investment_interest)
        monthly_saving *= 1 + salary_increase

    print("After {} years:".format(year))
    print("Monthly saving: {:,}".format(int(monthly_saving)))
    print("Networth: {:,}".format(int(net_worth)))
    print("Retirement monthly income: {:,}".format(int(net_worth * 0.04 / 12)))

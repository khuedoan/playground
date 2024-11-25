import numpy as np
import matplotlib.pyplot as plt
import datetime

# Simulation parameters
num_builds = 400
total_duration = 60 * 60 * 4
cron_interval = 180

def generate_weighted_random():
    # Define the ranges and their corresponding probabilities
    ranges = [30, 60, 180, 300]
    probabilities = [0.50, 0.90, 0.99, 1.00]  # Cumulative probabilities

    # Generate a random number between 0 and 1
    random_value = np.random.rand()

    # Determine the range based on the random value
    if random_value <= probabilities[0]:  # 50% for 1-30
        return np.random.randint(1, ranges[0] + 1)  # 1 to 30
    elif random_value <= probabilities[1]:  # 90% for 1-60
        return np.random.randint(1, ranges[1] + 1)  # 1 to 60
    elif random_value <= probabilities[2]:  # 99% for 1-90
        return np.random.randint(1, ranges[2] + 1)  # 1 to 90
    else:  # 100% for 1-180
        return np.random.randint(1, ranges[-1] + 1)  # 1 to 180

def generate_build_times(num_samples, total_duration):
    build_times = []
    
    # Start the first build at time 0
    current_time = 0
    
    for _ in range(num_samples):
        duration = generate_weighted_random()

        # Set start time to current time and calculate end time
        start_time = current_time
        end_time = start_time + duration

        # Append the tuple (start, finish) to the list
        build_times.append((start_time, end_time))
        
        # Update current time for the next build
        current_time = end_time

        # Generate random sleep time between builds
        sleep_time = np.random.uniform(60, 60 * 5)
        current_time += sleep_time  # Add sleep time to current time

        # If the end time exceeds total_duration, stop adding more builds
        if current_time > total_duration:
            break
    
    return build_times

def simulate_cron_jobs(build_times, cron_interval, total_duration):
    cron_jobs = np.arange(0, total_duration, cron_interval)

    return cron_jobs

def plot_builds_and_cron(build_times, cron_jobs):
    plt.figure(figsize=(12, 6))

    # Y-coordinates for builds and cron jobs
    build_y = 1
    cron_y = 0

    total_builds = 0
    ontime_builds = 0

    # Plot each build
    for start_time, end_time in build_times:
        # Check if the build was missed by any cron job
        is_missed = any(start_time < cron_job < end_time for cron_job in cron_jobs)  # Check if cron job falls within the build time
        color = 'green' if not is_missed else 'yellow'  # Green if synced, yellow if missed

        total_builds += 1

        if not is_missed:
            ontime_builds += 1

        plt.plot([start_time / 60, end_time / 60], [build_y, build_y], color=color, linewidth=10, label='Build' if start_time == build_times[0][0] else "")
    
    # Plot cron jobs with blue dots
    plt.plot(cron_jobs / 60, [cron_y] * len(cron_jobs), 'bo', label='GitOps poll')

    # Add horizontal lines for each cron job for easier comparison
    for cron_job in cron_jobs:
        plt.axhline(y=cron_y, xmin=0, xmax=(cron_job / 60), color='lightgray', linestyle='--', linewidth=0.8)

    ontime_percentage = (ontime_builds / total_builds) * 100

    # Add text annotations
    plt.text(0.5, 1.5, f'Percentage of on-time sync (same as polling directly from git): {ontime_percentage:.2f}%')

    plt.tight_layout()
    plt.axhline(y=0, color='black', linestyle='--', label='GitOps poll Line')
    plt.title('Build Times and GitOps poll')
    plt.xlabel('Time (minutes)')
    plt.xticks(ticks=np.arange(0, total_duration / 60 + 1, 3))  # 3-minute intervals, same as the cron
    plt.yticks([])
    plt.ylim(-1, 2)  # Adjust y-limits to bring lines closer
    plt.legend()
    plt.grid()
    plt.tight_layout()
    plt.show()

build_times = generate_build_times(num_builds, total_duration)

cron_jobs = simulate_cron_jobs(build_times, cron_interval, total_duration)

plot_builds_and_cron(build_times, cron_jobs)

#!/usr/bin/env python
"""
Test script for Moirai Engine Python bindings
"""

import sys
from moirai_engine.core import Engine, Job

def main():
    print("Creating Moirai Engine")
    engine = Engine()
    
    print("Creating sample job")
    job = Job(
        job_id="sample-job-001", 
        name="Sample Job", 
        description="A sample job to test Python bindings",
        timeout=60,
        retries=0,
        callback_url=""
    )
    
    # Add start node
    job.add_node(
        id="start",
        name="Start",
        action="start",
        plugin="system",
        version="1.0.0",
        x=100.0,
        y=100.0,
        timeout=10
    )
    
    # Add end node
    job.add_node(
        id="end",
        name="End",
        action="end",
        plugin="system",
        version="1.0.0",
        x=300.0,
        y=100.0,
        timeout=10
    )
    
    # Connect start to end
    job.add_edge(
        from_node="start",
        from_port="success",
        to_node="end",
        to_port="trigger",
        condition=None
    )
    
    # Validate job structure
    print("Validating job")
    try:
        job.validate()
        print("Job validation successful")
    except ValueError as e:
        print(f"Job validation failed: {e}")
        return
    
    # Print job JSON for debugging
    print("\nJob JSON:")
    print(job.to_json())
    
    # Submit job for execution
    print("\nSubmitting job")
    try:
        job_id = engine.submit_job(job)
        print(f"Job submitted successfully, ID: {job_id}")
    except RuntimeError as e:
        print(f"Job submission failed: {e}")
        return
    
    # Get job status
    print("\nChecking job status")
    status = engine.get_job_status(job_id)
    print(f"Current job status: {status}")
    
    print("\nPython bindings test completed successfully")

if __name__ == "__main__":
    main()
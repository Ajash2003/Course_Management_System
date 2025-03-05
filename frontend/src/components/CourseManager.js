import React, { useState } from 'react';
import axios from 'axios';

const API_URL = 'http://localhost:8000/api';

function CourseManager() {
  const [credits, setCredits] = useState('');
  const [result, setResult] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    setResult(null);

    try {
      const response = await axios.post(`${API_URL}/courses/delete`, {
        credits: parseInt(credits, 10)
      }, {
        headers: {
          'Content-Type': 'application/json',
        },
        withCredentials: false
      });
      
      setResult(response.data);
    } catch (err) {
      setError(err.response?.data?.message || 'An error occurred while deleting courses.');
      console.error('Error:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mt-5">
      <h1 className="mb-4">Course Management System</h1>
      
      <div className="card mb-4">
        <div className="card-header">Delete Courses by Credits</div>
        <div className="card-body">
          <form onSubmit={handleSubmit}>
            <div className="mb-3">
              <label htmlFor="credits" className="form-label">Credits Value:</label>
              <input
                type="number"
                className="form-control"
                id="credits"
                value={credits}
                onChange={(e) => setCredits(e.target.value)}
                min="1"
                required
              />
            </div>
            <button type="submit" className="btn btn-danger" disabled={loading}>
              {loading ? 'Processing...' : 'Delete Courses'}
            </button>
          </form>
        </div>
      </div>

      {error && (
        <div className="alert alert-danger" role="alert">
          {error}
        </div>
      )}

      {result && (
        <div className="mt-4">
          <div className={`alert ${result.success ? 'alert-success' : 'alert-danger'}`}>
            {result.message}
          </div>
          
          {result.data && result.data.deleted_courses.length > 0 && (
            <div>
              <h3>Deleted Courses:</h3>
              <table className="table table-striped">
                <thead>
                  <tr>
                    <th>ID</th>
                    <th>Title</th>
                    <th>Code</th>
                    <th>Credits</th>
                    <th>Department</th>
                    <th>Description</th>
                  </tr>
                </thead>
                <tbody>
                  {result.data.deleted_courses.map(course => (
                    <tr key={course.id}>
                      <td>{course.id}</td>
                      <td>{course.title}</td>
                      <td>{course.code}</td>
                      <td>{course.credits}</td>
                      <td>{course.department}</td>
                      <td>{course.description || '-'}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default CourseManager;
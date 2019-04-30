FROM rust:1.32.0-slim

# Add non-root user
RUN groupadd -g 1100 appuser && \
    useradd -r -u 1100 -g appuser -m -d /home/appuser appuser

RUN mkdir -p /var/www/app/legicash-mkb/
# Set permissions for appuser
RUN chown -R appuser:appuser /var/www/app/legicash-mkb/

# Install dependecies
RUN apt-get update -y
RUN apt-get install -y supervisor

# Create supervisor log file
#RUN touch /tmp/supervisord.log && \
RUN   chown appuser:appuser /var/log/supervisor /var/run/

# Set working directory and user
WORKDIR /var/www/app/legicash-mkb/

USER appuser

